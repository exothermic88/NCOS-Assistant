pub mod chunker;
pub mod index;

use futures_util::Stream;
use std::collections::BTreeMap;
use std::path::Path;

use crate::config::AssistantConfig;
use crate::ollama::{OllamaClient, OllamaError};
use chunker::Chunk;
use index::{IndexEntry, StoredIndex, INDEX_FORMAT_VERSION};

const EMBED_BATCH: usize = 16;

#[derive(Debug, Clone)]
pub enum IndexEvent {
    Progress { done: usize, total: usize },
    Finished(StoredIndex),
    Failed(String),
}

/// Absolute path -> mtime for every indexable file under the docs folders.
pub fn scan_files(docs_paths: &[String]) -> BTreeMap<String, u64> {
    let mut files = BTreeMap::new();
    for root in docs_paths {
        for entry in walkdir::WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown" | "txt") {
                continue;
            }
            let mtime = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            files.insert(path.to_string_lossy().into_owned(), mtime);
        }
    }
    files
}

pub fn needs_rebuild(existing: Option<&StoredIndex>, cfg: &AssistantConfig) -> bool {
    match existing {
        None => true,
        Some(idx) => idx.embed_model != cfg.embed_model || idx.files != scan_files(&cfg.docs_paths),
    }
}

fn collect_chunks(files: &BTreeMap<String, u64>, docs_paths: &[String]) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    for path in files.keys() {
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        // Present the source as a path relative to whichever docs root holds it.
        let source = docs_paths
            .iter()
            .find_map(|root| {
                Path::new(path)
                    .strip_prefix(root)
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| path.clone());
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext == "txt" {
            chunks.extend(chunker::chunk_plaintext(&source, &content));
        } else {
            chunks.extend(chunker::chunk_markdown(&source, &content));
        }
    }
    chunks
}

/// Build the index, yielding progress events and ending with Finished/Failed.
/// The finished index is also persisted to disk.
pub fn build_index(
    client: OllamaClient,
    cfg: AssistantConfig,
) -> impl Stream<Item = IndexEvent> + Send {
    async_stream::stream! {
        let files = scan_files(&cfg.docs_paths);
        let chunks = collect_chunks(&files, &cfg.docs_paths);
        let total = chunks.len();
        if total == 0 {
            yield IndexEvent::Failed(format!(
                "No .md or .txt files found in: {}",
                cfg.docs_paths.join(", ")
            ));
            return;
        }
        yield IndexEvent::Progress { done: 0, total };

        let mut entries = Vec::with_capacity(total);
        for batch in chunks.chunks(EMBED_BATCH) {
            let inputs: Vec<String> = batch.iter().map(|c| c.text.clone()).collect();
            match client.embed(&cfg.embed_model, inputs).await {
                Ok(vectors) => {
                    if vectors.len() != batch.len() {
                        yield IndexEvent::Failed("embedding count mismatch".into());
                        return;
                    }
                    for (chunk, mut vector) in batch.iter().zip(vectors) {
                        index::normalize(&mut vector);
                        entries.push(IndexEntry {
                            source: chunk.source.clone(),
                            heading_path: chunk.heading_path.clone(),
                            text: chunk.text.clone(),
                            embedding: vector,
                        });
                    }
                    yield IndexEvent::Progress { done: entries.len(), total };
                }
                Err(e) => {
                    yield IndexEvent::Failed(e.to_string());
                    return;
                }
            }
        }

        let stored = StoredIndex {
            version: INDEX_FORMAT_VERSION,
            embed_model: cfg.embed_model.clone(),
            files,
            entries,
        };
        if let Err(e) = index::save(&stored) {
            yield IndexEvent::Failed(format!("failed to save index: {e}"));
            return;
        }
        yield IndexEvent::Finished(stored);
    }
}

/// Embed the query and return the top-k most relevant chunks.
pub async fn retrieve<'a>(
    client: &OllamaClient,
    stored: &'a StoredIndex,
    cfg: &AssistantConfig,
    query: &str,
) -> Result<Vec<&'a IndexEntry>, OllamaError> {
    let mut vectors = client
        .embed(&cfg.embed_model, vec![query.to_string()])
        .await?;
    let Some(mut query_vec) = vectors.pop() else {
        return Err(OllamaError::Other("empty embedding response".into()));
    };
    index::normalize(&mut query_vec);
    Ok(index::top_k(&query_vec, &stored.entries, cfg.top_k as usize))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[test]
    fn chunker_splits_on_headings() {
        let md = "# Title\n\nIntro text long enough to stand alone. Lorem ipsum dolor sit amet, \
            consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore \
            magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco.\n\n\
            ## Updating\n\nRun pacman -Syu to update the whole system. Partial upgrades are \
            unsupported on Arch-based systems, so never install packages after a bare -Sy \
            without doing the full system upgrade first. Reboot after kernel updates land.\n";
        let chunks = chunker::chunk_markdown("test.md", md);
        assert!(chunks.len() >= 2, "expected >=2 chunks, got {}", chunks.len());
        assert_eq!(chunks[0].heading_path, "Title");
        assert_eq!(chunks[1].heading_path, "Title > Updating");
        assert!(chunks[1].text.contains("pacman -Syu"));
    }

    /// Live test against local Ollama: index docs/ and retrieve.
    /// Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore]
    async fn live_index_and_retrieve() {
        let cfg = AssistantConfig::default();
        let (host, port) = cfg.active_backend();
        let client = OllamaClient::new(host, port);

        let mut stream = Box::pin(build_index(client.clone(), cfg.clone()));
        let mut finished = None;
        while let Some(event) = stream.next().await {
            match event {
                IndexEvent::Finished(idx) => finished = Some(idx),
                IndexEvent::Failed(e) => panic!("index build failed: {e}"),
                IndexEvent::Progress { .. } => {}
            }
        }
        let stored = finished.expect("no Finished event");
        assert!(!stored.entries.is_empty());

        let hits = retrieve(&client, &stored, &cfg, "how do I update the system?")
            .await
            .expect("retrieve failed");
        assert!(!hits.is_empty());
        assert_eq!(
            hits[0].heading_path, "NCOS Overview > Updating the system",
            "top hit was {} > {}",
            hits[0].source, hits[0].heading_path
        );
    }

    /// Live test of streaming chat parsing using the tiny gemma3:270m model.
    #[tokio::test]
    #[ignore]
    async fn live_chat_stream() {
        let client = OllamaClient::new("localhost", 11434);
        let stream = client
            .chat_stream(
                "gemma3:270m".into(),
                vec![crate::ollama::ChatMessage::new("user", "Say hello in one word.")],
            )
            .await
            .expect("chat_stream failed");
        futures_util::pin_mut!(stream);
        let mut out = String::new();
        while let Some(item) = stream.next().await {
            out.push_str(&item.expect("stream item error"));
        }
        assert!(!out.is_empty());
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

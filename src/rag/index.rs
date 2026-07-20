use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub const INDEX_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub source: String,
    pub heading_path: String,
    pub text: String,
    /// L2-normalized, so cosine similarity is a plain dot product.
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoredIndex {
    pub version: u32,
    pub embed_model: String,
    /// Absolute file path -> mtime (unix secs) at index time.
    pub files: BTreeMap<String, u64>,
    pub entries: Vec<IndexEntry>,
}

pub fn index_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("ncos-assistant")
        .join("index.json")
}

pub fn load() -> Option<StoredIndex> {
    let raw = std::fs::read_to_string(index_path()).ok()?;
    let index: StoredIndex = serde_json::from_str(&raw).ok()?;
    (index.version == INDEX_FORMAT_VERSION).then_some(index)
}

pub fn save(index: &StoredIndex) -> std::io::Result<()> {
    let path = index_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let raw = serde_json::to_string(index).map_err(std::io::Error::other)?;
    std::fs::write(path, raw)
}

pub fn normalize(v: &mut [f32]) {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in v {
            *x /= norm;
        }
    }
}

/// Top-k entries by dot product (== cosine on normalized vectors).
pub fn top_k<'a>(query: &[f32], entries: &'a [IndexEntry], k: usize) -> Vec<&'a IndexEntry> {
    let mut scored: Vec<(f32, &IndexEntry)> = entries
        .iter()
        .map(|e| {
            let score: f32 = e
                .embedding
                .iter()
                .zip(query)
                .map(|(a, b)| a * b)
                .sum();
            (score, e)
        })
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.into_iter().take(k).map(|(_, e)| e).collect()
}

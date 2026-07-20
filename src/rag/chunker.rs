/// A chunk of documentation ready for embedding.
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Path relative to the docs root it was found in.
    pub source: String,
    /// "Networking > Wi-Fi setup" style breadcrumb of headings.
    pub heading_path: String,
    /// Chunk body, heading path included, as sent to the embedder.
    pub text: String,
}

const MIN_CHUNK_CHARS: usize = 200;
const MAX_CHUNK_CHARS: usize = 1500;

/// Split a markdown document into heading-aware chunks (h1-h3 boundaries).
/// Heading markers inside fenced code blocks are ignored. Sections shorter
/// than MIN_CHUNK_CHARS merge into the previous chunk; longer than
/// MAX_CHUNK_CHARS split at paragraph boundaries.
pub fn chunk_markdown(source: &str, content: &str) -> Vec<Chunk> {
    let mut sections: Vec<(Vec<String>, String)> = Vec::new();
    let mut heading_stack: Vec<(u8, String)> = Vec::new();
    let mut current = String::new();
    let mut in_fence = false;

    let flush = |sections: &mut Vec<(Vec<String>, String)>,
                 heading_stack: &[(u8, String)],
                 current: &mut String| {
        let body = current.trim().to_string();
        if !body.is_empty() {
            let path = heading_stack.iter().map(|(_, h)| h.clone()).collect();
            sections.push((path, body));
        }
        current.clear();
    };

    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            current.push_str(line);
            current.push('\n');
            continue;
        }
        let heading_level = if !in_fence && trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            if (1..=3).contains(&level) && trimmed.chars().nth(level) == Some(' ') {
                Some(level as u8)
            } else {
                None
            }
        } else {
            None
        };

        match heading_level {
            Some(level) => {
                flush(&mut sections, &heading_stack, &mut current);
                let title = trimmed[level as usize..].trim().to_string();
                heading_stack.retain(|(l, _)| *l < level);
                heading_stack.push((level, title));
            }
            None => {
                current.push_str(line);
                current.push('\n');
            }
        }
    }
    flush(&mut sections, &heading_stack, &mut current);

    // Merge tiny sections into their predecessor, split oversized ones.
    let mut chunks: Vec<Chunk> = Vec::new();
    for (path, body) in sections {
        let heading_path = path.join(" > ");
        if body.len() < MIN_CHUNK_CHARS {
            if let Some(prev) = chunks.last_mut() {
                prev.text.push_str("\n\n");
                if !heading_path.is_empty() {
                    prev.text.push_str(&format!("## {heading_path}\n"));
                }
                prev.text.push_str(&body);
                continue;
            }
        }
        for part in split_long(&body) {
            chunks.push(make_chunk(source, &heading_path, &part));
        }
    }
    chunks
}

/// Whole-file chunking for plain-text files.
pub fn chunk_plaintext(source: &str, content: &str) -> Vec<Chunk> {
    split_long(content.trim())
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(|part| make_chunk(source, "", &part))
        .collect()
}

fn make_chunk(source: &str, heading_path: &str, body: &str) -> Chunk {
    let text = if heading_path.is_empty() {
        format!("[{source}]\n{body}")
    } else {
        format!("[{source} > {heading_path}]\n{body}")
    };
    Chunk {
        source: source.to_string(),
        heading_path: heading_path.to_string(),
        text,
    }
}

fn split_long(body: &str) -> Vec<String> {
    if body.len() <= MAX_CHUNK_CHARS {
        return vec![body.to_string()];
    }
    let mut parts = Vec::new();
    let mut current = String::new();
    for para in body.split("\n\n") {
        if !current.is_empty() && current.len() + para.len() > MAX_CHUNK_CHARS {
            parts.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push_str("\n\n");
        }
        current.push_str(para);
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

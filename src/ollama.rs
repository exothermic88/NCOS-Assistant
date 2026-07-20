use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: &str, content: impl Into<String>) -> Self {
        Self {
            role: role.to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OllamaError {
    Unreachable(String),
    ModelMissing(String),
    Other(String),
}

impl std::fmt::Display for OllamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreachable(url) => write!(
                f,
                "Can't reach Ollama at {url}. Start it with: systemctl enable --now ollama"
            ),
            Self::ModelMissing(model) => {
                write!(f, "Model '{model}' is not installed. Run: ollama pull {model}")
            }
            Self::Other(err) => write!(f, "Ollama error: {err}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OllamaClient {
    base: String,
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct ChatChunk {
    #[serde(default)]
    message: Option<ChatMessage>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

impl OllamaClient {
    pub fn new(host: &str, port: u16) -> Self {
        let host = host.trim_end_matches('/');
        let base = if host.starts_with("http://") || host.starts_with("https://") {
            format!("{host}:{port}")
        } else {
            format!("http://{host}:{port}")
        };
        Self {
            base,
            http: reqwest::Client::new(),
        }
    }

    fn map_send_error(&self, err: reqwest::Error) -> OllamaError {
        if err.is_connect() || err.is_timeout() {
            OllamaError::Unreachable(self.base.clone())
        } else {
            OllamaError::Other(err.to_string())
        }
    }

    async fn check_status(&self, resp: reqwest::Response, model: &str) -> Result<reqwest::Response, OllamaError> {
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let detail = serde_json::from_str::<ErrorResponse>(&body)
            .map(|e| e.error)
            .unwrap_or(body);
        if status == reqwest::StatusCode::NOT_FOUND && detail.contains("not found") {
            Err(OllamaError::ModelMissing(model.to_string()))
        } else {
            Err(OllamaError::Other(detail))
        }
    }

    /// Streaming chat via /api/chat; yields content deltas.
    pub async fn chat_stream(
        &self,
        model: String,
        messages: Vec<ChatMessage>,
    ) -> Result<impl Stream<Item = Result<String, OllamaError>> + Send + use<>, OllamaError> {
        // No `think` parameter: on current Ollama, `think: false` makes
        // thinking models leak reasoning into `content`, while the default
        // routes it to the separate `thinking` field, which we skip.
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
        });
        let resp = self
            .http
            .post(format!("{}/api/chat", self.base))
            .json(&body)
            .send()
            .await
            .map_err(|e| self.map_send_error(e))?;
        let resp = self.check_status(resp, &model).await?;

        let mut bytes = resp.bytes_stream();
        Ok(async_stream::stream! {
            let mut buf = Vec::new();
            while let Some(chunk) = bytes.next().await {
                match chunk {
                    Ok(chunk) => buf.extend_from_slice(&chunk),
                    Err(e) => {
                        yield Err(OllamaError::Other(e.to_string()));
                        return;
                    }
                }
                while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                    let line: Vec<u8> = buf.drain(..=pos).collect();
                    let line = String::from_utf8_lossy(&line);
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    match serde_json::from_str::<ChatChunk>(line) {
                        Ok(parsed) => {
                            if let Some(err) = parsed.error {
                                yield Err(OllamaError::Other(err));
                                return;
                            }
                            if let Some(msg) = parsed.message {
                                if !msg.content.is_empty() {
                                    yield Ok(msg.content);
                                }
                            }
                            if parsed.done {
                                return;
                            }
                        }
                        Err(e) => {
                            yield Err(OllamaError::Other(format!("bad response line: {e}")));
                            return;
                        }
                    }
                }
            }
        })
    }

    /// Batch embeddings via /api/embed.
    pub async fn embed(&self, model: &str, inputs: Vec<String>) -> Result<Vec<Vec<f32>>, OllamaError> {
        let resp = self
            .http
            .post(format!("{}/api/embed", self.base))
            .json(&serde_json::json!({
                "model": model,
                "input": inputs,
            }))
            .send()
            .await
            .map_err(|e| self.map_send_error(e))?;
        let resp = self.check_status(resp, model).await?;
        let parsed: EmbedResponse = resp
            .json()
            .await
            .map_err(|e| OllamaError::Other(e.to_string()))?;
        Ok(parsed.embeddings)
    }

    /// Quick reachability probe.
    pub async fn health(&self) -> bool {
        self.http
            .get(format!("{}/api/version", self.base))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}

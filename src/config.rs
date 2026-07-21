use cosmic::cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use serde::{Deserialize, Serialize};

use crate::app::APP_ID;

pub const CONFIG_VERSION: u64 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CosmicConfigEntry)]
#[version = 1]
pub struct AssistantConfig {
    pub use_remote: bool,
    pub local_host: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub chat_model: String,
    pub embed_model: String,
    pub docs_paths: Vec<String>,
    pub top_k: u32,
    pub show_sources: bool,
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            use_remote: false,
            local_host: "localhost".into(),
            local_port: 11434,
            remote_host: String::new(),
            remote_port: 11434,
            chat_model: "qwen3:4b".into(),
            embed_model: "nomic-embed-text".into(),
            docs_paths: vec!["/usr/share/ncos-assistant/docs".into()],
            top_k: 4,
            show_sources: true,
        }
    }
}

impl AssistantConfig {
    pub fn handle() -> Option<cosmic_config::Config> {
        cosmic_config::Config::new(APP_ID, CONFIG_VERSION).ok()
    }

    pub fn load() -> Self {
        Self::handle()
            .map(|config| {
                Self::get_entry(&config).unwrap_or_else(|(_errors, entry)| entry)
            })
            .unwrap_or_default()
    }

    /// Host/port of whichever backend is active.
    pub fn active_backend(&self) -> (&str, u16) {
        if self.use_remote && !self.remote_host.is_empty() {
            (&self.remote_host, self.remote_port)
        } else {
            (&self.local_host, self.local_port)
        }
    }
}

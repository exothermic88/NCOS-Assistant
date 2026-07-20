use std::sync::Arc;

use cosmic::app::{Core, Task};
use cosmic::cosmic_config;
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription};
use cosmic::widget;
use cosmic::{Application, Element};
use futures_util::{Stream, StreamExt};

use crate::config::AssistantConfig;
use crate::ollama::{ChatMessage, OllamaClient};
use crate::rag::{self, index::StoredIndex, IndexEvent};

pub const APP_ID: &str = "io.github.exothermic88.ncos-assistant";

pub fn input_id() -> widget::Id {
    widget::Id::new("chat-input")
}

const SYSTEM_PROMPT: &str = "You are the ncOS Assistant, a helper for ncOS, an Arch Linux-based \
distribution using the COSMIC desktop environment. Answer the user's questions concisely and \
accurately. Prefer the documentation context below when it covers the question, and cite sources \
inline like [source: file.md > heading]. If the documentation does not cover the question, say so \
briefly, then give your best general Arch Linux / COSMIC answer.";

/// How many prior chat entries are replayed to the model each turn.
const HISTORY_WINDOW: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ChatEntry {
    pub role: Role,
    pub content: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ChatEvent {
    Sources(Vec<String>),
    Token(String),
    Done,
    Failed(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum IndexStatus {
    #[default]
    NotBuilt,
    Building {
        done: usize,
        total: usize,
    },
    Ready {
        chunks: usize,
    },
    Failed(String),
}

#[derive(Default)]
pub struct NcosAssistant {
    core: Core,
    popup: Option<Id>,
    config: AssistantConfig,
    config_handler: Option<cosmic_config::Config>,
    pub input: String,
    pub history: Vec<ChatEntry>,
    pub streaming: bool,
    abort: Option<cosmic::iced::task::Handle>,
    index: Option<Arc<StoredIndex>>,
    pub index_status: IndexStatus,
    pub backend_ok: Option<bool>,
    pub show_settings: bool,
    pub last_error: Option<String>,
    pub remote_port_text: String,
    pub new_docs_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    InputChanged(String),
    Send,
    StopGeneration,
    ClearChat,
    Chat(ChatEvent),
    HealthChecked(bool),
    RebuildIndex,
    Index(IndexEvent),
    ToggleSettings,
    ConfigChanged(AssistantConfig),
    SetUseRemote(bool),
    SetRemoteHost(String),
    SetRemotePort(String),
    SetChatModel(String),
    SetEmbedModel(String),
    SetTopK(u32),
    NewDocsPathChanged(String),
    AddDocsPath,
    RemoveDocsPath(usize),
}

impl NcosAssistant {
    pub fn config(&self) -> &AssistantConfig {
        &self.config
    }

    fn client(&self) -> OllamaClient {
        let (host, port) = self.config.active_backend();
        OllamaClient::new(host, port)
    }

    fn save_config(&mut self, edit: impl FnOnce(&mut AssistantConfig)) {
        edit(&mut self.config);
        if let Some(handler) = &self.config_handler {
            let _ = self.config.write_entry(handler);
        }
    }

    fn health_task(&self) -> Task<Message> {
        let client = self.client();
        cosmic::task::future(async move { Message::HealthChecked(client.health().await) })
    }

    fn rebuild_task(&mut self) -> Task<Message> {
        self.index_status = IndexStatus::Building { done: 0, total: 0 };
        let stream = rag::build_index(self.client(), self.config.clone());
        cosmic::iced::Task::stream(stream.map(|ev| cosmic::action::app(Message::Index(ev))))
    }

    fn send_task(&mut self) -> Task<Message> {
        let prompt = self.input.trim().to_string();
        if prompt.is_empty() || self.streaming {
            return Task::none();
        }
        self.input.clear();
        self.last_error = None;

        let history: Vec<ChatMessage> = self
            .history
            .iter()
            .rev()
            .take(HISTORY_WINDOW)
            .rev()
            .filter(|e| !e.content.is_empty())
            .map(|e| {
                ChatMessage::new(
                    match e.role {
                        Role::User => "user",
                        Role::Assistant => "assistant",
                    },
                    &e.content,
                )
            })
            .collect();

        self.history.push(ChatEntry {
            role: Role::User,
            content: prompt.clone(),
            sources: Vec::new(),
        });
        self.history.push(ChatEntry {
            role: Role::Assistant,
            content: String::new(),
            sources: Vec::new(),
        });
        self.streaming = true;

        let stream = chat_flow(
            self.client(),
            self.config.clone(),
            self.index.clone(),
            history,
            prompt,
        );
        let (task, handle) = cosmic::iced::Task::stream(
            stream.map(|ev| cosmic::action::app(Message::Chat(ev))),
        )
        .abortable();
        self.abort = Some(handle);
        task
    }
}

impl Application for NcosAssistant {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let config = AssistantConfig::load();
        let app = Self {
            core,
            config_handler: AssistantConfig::handle(),
            remote_port_text: config.remote_port.to_string(),
            index: rag::index::load().map(Arc::new),
            config,
            ..Default::default()
        };
        let mut app = app;
        app.index_status = match &app.index {
            Some(index) => IndexStatus::Ready {
                chunks: index.entries.len(),
            },
            None => IndexStatus::NotBuilt,
        };
        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Message> {
        struct ConfigSub;
        cosmic_config::config_subscription(
            std::any::TypeId::of::<ConfigSub>(),
            APP_ID.into(),
            crate::config::CONFIG_VERSION,
        )
        .map(|update: cosmic_config::Update<AssistantConfig>| {
            Message::ConfigChanged(update.config)
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TogglePopup => {
                if let Some(id) = self.popup.take() {
                    return destroy_popup(id);
                }
                let new_id = Id::unique();
                self.popup = Some(new_id);
                let mut popup_settings = self.core.applet.get_popup_settings(
                    self.core.main_window_id().unwrap(),
                    new_id,
                    None,
                    None,
                    None,
                );
                popup_settings.positioner.size_limits = Limits::NONE
                    .min_width(380.0)
                    .max_width(440.0)
                    .min_height(300.0)
                    .max_height(640.0);

                let mut tasks = vec![
                    get_popup(popup_settings),
                    self.health_task(),
                    widget::text_input::focus(input_id()),
                ];
                let stale = !matches!(self.index_status, IndexStatus::Building { .. })
                    && rag::needs_rebuild(self.index.as_deref(), &self.config);
                if stale {
                    tasks.push(self.rebuild_task());
                }
                Task::batch(tasks)
            }
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::InputChanged(input) => {
                self.input = input;
                Task::none()
            }
            Message::Send => self.send_task(),
            Message::StopGeneration => {
                if let Some(handle) = self.abort.take() {
                    handle.abort();
                }
                self.streaming = false;
                Task::none()
            }
            Message::ClearChat => {
                if let Some(handle) = self.abort.take() {
                    handle.abort();
                }
                self.streaming = false;
                self.history.clear();
                self.last_error = None;
                Task::none()
            }
            Message::Chat(event) => {
                match event {
                    ChatEvent::Sources(sources) => {
                        if let Some(last) = self.history.last_mut() {
                            last.sources = sources;
                        }
                    }
                    ChatEvent::Token(token) => {
                        if let Some(last) = self.history.last_mut() {
                            last.content.push_str(&token);
                        }
                    }
                    ChatEvent::Done => {
                        self.streaming = false;
                        self.abort = None;
                    }
                    ChatEvent::Failed(err) => {
                        self.streaming = false;
                        self.abort = None;
                        self.last_error = Some(err);
                        // Drop the empty assistant bubble on failure.
                        if self.history.last().is_some_and(|e| {
                            e.role == Role::Assistant && e.content.is_empty()
                        }) {
                            self.history.pop();
                        }
                    }
                }
                Task::none()
            }
            Message::HealthChecked(ok) => {
                self.backend_ok = Some(ok);
                Task::none()
            }
            Message::RebuildIndex => self.rebuild_task(),
            Message::Index(event) => {
                match event {
                    IndexEvent::Progress { done, total } => {
                        self.index_status = IndexStatus::Building { done, total };
                    }
                    IndexEvent::Finished(stored) => {
                        self.index_status = IndexStatus::Ready {
                            chunks: stored.entries.len(),
                        };
                        self.index = Some(Arc::new(stored));
                    }
                    IndexEvent::Failed(err) => {
                        self.index_status = IndexStatus::Failed(err);
                    }
                }
                Task::none()
            }
            Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                Task::none()
            }
            Message::ConfigChanged(config) => {
                self.remote_port_text = config.remote_port.to_string();
                self.config = config;
                self.health_task()
            }
            Message::SetUseRemote(value) => {
                self.save_config(|c| c.use_remote = value);
                self.health_task()
            }
            Message::SetRemoteHost(value) => {
                self.save_config(|c| c.remote_host = value);
                Task::none()
            }
            Message::SetRemotePort(value) => {
                self.remote_port_text = value.clone();
                if let Ok(port) = value.parse::<u16>() {
                    self.save_config(|c| c.remote_port = port);
                }
                Task::none()
            }
            Message::SetChatModel(value) => {
                self.save_config(|c| c.chat_model = value);
                Task::none()
            }
            Message::SetEmbedModel(value) => {
                self.save_config(|c| c.embed_model = value);
                Task::none()
            }
            Message::SetTopK(value) => {
                self.save_config(|c| c.top_k = value);
                Task::none()
            }
            Message::NewDocsPathChanged(value) => {
                self.new_docs_path = value;
                Task::none()
            }
            Message::AddDocsPath => {
                let path = self.new_docs_path.trim().to_string();
                if !path.is_empty() {
                    self.save_config(|c| {
                        if !c.docs_paths.contains(&path) {
                            c.docs_paths.push(path);
                        }
                    });
                    self.new_docs_path.clear();
                }
                Task::none()
            }
            Message::RemoveDocsPath(i) => {
                self.save_config(|c| {
                    if i < c.docs_paths.len() {
                        c.docs_paths.remove(i);
                    }
                });
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.core
            .applet
            .icon_button(APP_ID)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: Id) -> Element<'_, Message> {
        if self.popup != Some(id) {
            return widget::text("").into();
        }
        let content = if self.show_settings {
            self.settings_view()
        } else {
            self.chat_view()
        };
        self.core.applet.popup_container(content).into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

/// Retrieval + streaming chat as one abortable event stream.
fn chat_flow(
    client: OllamaClient,
    cfg: AssistantConfig,
    index: Option<Arc<StoredIndex>>,
    history: Vec<ChatMessage>,
    prompt: String,
) -> impl Stream<Item = ChatEvent> + Send {
    async_stream::stream! {
        let mut system = SYSTEM_PROMPT.to_string();
        if let Some(index) = &index {
            if !index.entries.is_empty() {
                match rag::retrieve(&client, index, &cfg, &prompt).await {
                    Ok(hits) if !hits.is_empty() => {
                        system.push_str("\n\nDocumentation context:\n");
                        let mut sources = Vec::new();
                        for hit in &hits {
                            system.push_str(&format!("\n{}\n", hit.text));
                            let label = if hit.heading_path.is_empty() {
                                hit.source.clone()
                            } else {
                                format!("{} > {}", hit.source, hit.heading_path)
                            };
                            if !sources.contains(&label) {
                                sources.push(label);
                            }
                        }
                        yield ChatEvent::Sources(sources);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        yield ChatEvent::Failed(e.to_string());
                        return;
                    }
                }
            }
        }

        let mut messages = vec![ChatMessage::new("system", &system)];
        messages.extend(history);
        messages.push(ChatMessage::new("user", &prompt));

        // qwen3 and deepseek-r1 support toggling thinking off; leave others alone.
        let think = (cfg.chat_model.starts_with("qwen3") || cfg.chat_model.starts_with("deepseek-r1"))
            .then_some(false);

        match client.chat_stream(cfg.chat_model.clone(), messages, think).await {
            Ok(stream) => {
                futures_util::pin_mut!(stream);
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(token) => yield ChatEvent::Token(token),
                        Err(e) => {
                            yield ChatEvent::Failed(e.to_string());
                            return;
                        }
                    }
                }
                yield ChatEvent::Done;
            }
            Err(e) => yield ChatEvent::Failed(e.to_string()),
        }
    }
}

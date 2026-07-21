use cosmic::iced::{Alignment, Length};
use cosmic::widget;
use cosmic::Element;

use crate::app::{IndexStatus, Message, NcosAssistant};

impl NcosAssistant {
    pub fn settings_view(&self) -> Element<'_, Message> {
        let spacing = cosmic::theme::spacing();
        let config = self.config();

        let header = widget::Row::new()
            .push(
                widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                    .on_press(Message::ToggleSettings),
            )
            .push(widget::text::title4("Settings"))
            .align_y(Alignment::Center)
            .spacing(spacing.space_xxs);

        let mut backend = widget::settings::section()
            .title("Ollama server")
            .add(widget::settings::item(
                "Use remote server",
                widget::toggler(config.use_remote).on_toggle(Message::SetUseRemote),
            ));
        if config.use_remote {
            backend = backend
                .add(widget::settings::item(
                    "Remote host",
                    widget::text_input("192.168.1.50", &config.remote_host)
                        .on_input(Message::SetRemoteHost)
                        .width(Length::Fixed(180.0)),
                ))
                .add(widget::settings::item(
                    "Remote port",
                    widget::text_input("11434", &self.remote_port_text)
                        .on_input(Message::SetRemotePort)
                        .width(Length::Fixed(90.0)),
                ));
        }

        let models = widget::settings::section()
            .title("Models")
            .add(widget::settings::item(
                "Chat model",
                widget::text_input("llama3.2:1b", &config.chat_model)
                    .on_input(Message::SetChatModel)
                    .width(Length::Fixed(180.0)),
            ))
            .add(widget::settings::item(
                "Embedding model",
                widget::text_input("embeddinggemma:300m", &config.embed_model)
                    .on_input(Message::SetEmbedModel)
                    .width(Length::Fixed(180.0)),
            ))
            .add(widget::settings::item(
                "Show sources",
                widget::toggler(config.show_sources).on_toggle(Message::SetShowSources),
            ))
            .add(widget::settings::item(
                "Context chunks",
                widget::spin_button(
                    config.top_k.to_string(),
                    config.top_k,
                    1,
                    1,
                    12,
                    Message::SetTopK,
                ),
            ));

        let mut docs = widget::settings::section().title("Documentation folders");
        for (i, path) in config.docs_paths.iter().enumerate() {
            docs = docs.add(widget::settings::item(
                path.as_str(),
                widget::button::icon(widget::icon::from_name("user-trash-symbolic"))
                    .on_press(Message::RemoveDocsPath(i)),
            ));
        }
        docs = docs.add(
            widget::Row::new()
                .push(
                    widget::text_input("/path/to/more/docs", &self.new_docs_path)
                        .on_input(Message::NewDocsPathChanged)
                        .on_submit(|_| Message::AddDocsPath),
                )
                .push(
                    widget::button::icon(widget::icon::from_name("list-add-symbolic"))
                        .on_press(Message::AddDocsPath),
                )
                .align_y(Alignment::Center)
                .spacing(spacing.space_xxs),
        );

        let index_label = match &self.index_status {
            IndexStatus::Building { done, total } if *total > 0 => {
                format!("Indexing {done}/{total}…")
            }
            IndexStatus::Building { .. } => "Indexing…".to_string(),
            IndexStatus::Ready { chunks } => format!("Index ready · {chunks} chunks"),
            IndexStatus::NotBuilt => "Index not built yet".to_string(),
            IndexStatus::Failed(err) => format!("Index failed: {err}"),
        };
        let index_section = widget::settings::section()
            .title("Document index")
            .add(widget::settings::item(
                index_label,
                widget::button::standard("Rebuild").on_press(Message::RebuildIndex),
            ));

        widget::scrollable(
            widget::Column::new()
                .push(header)
                .push(backend)
                .push(models)
                .push(docs)
                .push(index_section)
                .spacing(spacing.space_s)
                .padding(spacing.space_m)
                .width(Length::Fill),
        )
        .height(Length::Fixed(560.0))
        .into()
    }
}

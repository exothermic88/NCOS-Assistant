use cosmic::iced::{Alignment, Length};
use cosmic::widget;
use cosmic::Element;

use crate::app::{IndexStatus, Message, NcosAssistant, Role};

impl NcosAssistant {
    pub fn chat_view(&self) -> Element<'_, Message> {
        let spacing = cosmic::theme::spacing();

        let header = widget::Row::new()
            .push(widget::text::title4("ncOS Assistant"))
            .push(widget::Space::new().width(Length::Fill))
            .push(
                widget::button::icon(widget::icon::from_name("edit-clear-all-symbolic"))
                    .on_press(Message::ClearChat),
            )
            .push(
                widget::button::icon(widget::icon::from_name("emblem-system-symbolic"))
                    .on_press(Message::ToggleSettings),
            )
            .align_y(Alignment::Center)
            .spacing(spacing.space_xxs);

        let status = widget::text::caption(self.status_line());

        let mut history = widget::Column::new().spacing(spacing.space_s).width(Length::Fill);
        if self.history.is_empty() {
            history = history.push(
                widget::container(widget::text::body("Ask anything about ncOS."))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding([spacing.space_l, 0]),
            );
        }
        for entry in &self.history {
            match entry.role {
                Role::User => {
                    history = history.push(
                        widget::Row::new()
                            .push(widget::Space::new().width(Length::Fill))
                            .push(
                                widget::container(widget::text::body(&entry.content))
                                    .class(cosmic::style::Container::Card)
                                    .padding([spacing.space_xxs, spacing.space_s])
                                    .max_width(340.0),
                            ),
                    );
                }
                Role::Assistant => {
                    let text = if entry.content.is_empty() && self.streaming {
                        "…".to_string()
                    } else {
                        entry.content.clone()
                    };
                    let mut answer = widget::Column::new()
                        .push(widget::text::body(text))
                        .spacing(spacing.space_xxxs);
                    if !entry.sources.is_empty() {
                        answer = answer.push(widget::text::caption(format!(
                            "Sources: {}",
                            entry.sources.join(" · ")
                        )));
                    }
                    history = history.push(answer);
                }
            }
        }

        let scroll = widget::scrollable(
            widget::container(history).padding([0, spacing.space_xxs]),
        )
        .anchor_bottom()
        .height(Length::Fill)
        .width(Length::Fill);

        let mut input = widget::text_input("Ask about ncOS…", &self.input)
            .id(crate::app::input_id())
            .on_input(Message::InputChanged);
        if !self.streaming {
            input = input.on_submit(|_| Message::Send);
        }
        let action_button = if self.streaming {
            widget::button::icon(widget::icon::from_name("media-playback-stop-symbolic"))
                .on_press(Message::StopGeneration)
        } else {
            widget::button::icon(widget::icon::from_name("mail-send-symbolic"))
                .on_press(Message::Send)
        };
        let input_row = widget::Row::new()
            .push(input)
            .push(action_button)
            .align_y(Alignment::Center)
            .spacing(spacing.space_xxs);

        let mut content = widget::Column::new()
            .push(header)
            .push(status)
            .spacing(spacing.space_s)
            .padding(spacing.space_m)
            .width(Length::Fill)
            .height(Length::Fixed(560.0));

        if let Some(error) = &self.last_error {
            content = content.push(
                widget::container(widget::text::body(error))
                    .class(cosmic::style::Container::Card)
                    .padding(spacing.space_s)
                    .width(Length::Fill),
            );
        }

        content.push(scroll).push(input_row).into()
    }

    fn status_line(&self) -> String {
        let (host, port) = self.config().active_backend();
        let backend = match self.backend_ok {
            Some(true) => format!("Ollama @ {host}:{port} · connected"),
            Some(false) => format!("Ollama @ {host}:{port} · offline"),
            None => format!("Ollama @ {host}:{port}"),
        };
        let index = match &self.index_status {
            IndexStatus::NotBuilt => "no index".to_string(),
            IndexStatus::Building { done, total } if *total > 0 => {
                format!("indexing {done}/{total}…")
            }
            IndexStatus::Building { .. } => "indexing…".to_string(),
            IndexStatus::Ready { chunks } => format!("{chunks} doc chunks"),
            IndexStatus::Failed(_) => "index failed".to_string(),
        };
        format!("{backend} · {index}")
    }
}

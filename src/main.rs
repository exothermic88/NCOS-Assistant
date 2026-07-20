mod app;
mod config;
mod ollama;
mod rag;
mod ui;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<app::NcosAssistant>(())
}

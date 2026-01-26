mod app;
mod eyedropper;

use crate::config::Config;
use iced::{Result, window};

pub fn run(config: Config) -> Result {
    iced::application(
        "cascolor",
        app::CasColorApp::update,
        app::CasColorApp::view
    )
    .theme(app::CasColorApp::theme)
    .window(window::Settings {
        size: iced::Size::new(340.0, 680.0),
        min_size: Some(iced::Size::new(340.0, 600.0)),
        max_size: None,
        position: window::Position::Default,
        resizable: true,
        decorations: true,
        transparent: false,
        ..Default::default()
    })
    .antialiasing(true)
    .run_with(move || app::CasColorApp::new(config))
}

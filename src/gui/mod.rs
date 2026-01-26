mod app;
mod eyedropper;

use crate::config::Config;
use iced::{Result, window};

pub fn run(config: Config) -> Result {
    iced::application(
        "cascolor - Beautiful Color Picker",
        app::CasColorApp::update,
        app::CasColorApp::view
    )
    .theme(app::CasColorApp::theme)
    .window(window::Settings {
        size: iced::Size::new(420.0, 720.0),
        min_size: Some(iced::Size::new(350.0, 500.0)),
        max_size: Some(iced::Size::new(600.0, 900.0)),
        position: window::Position::Default,
        resizable: true,
        decorations: true,
        transparent: false,
        ..Default::default()
    })
    .antialiasing(true)
    .run_with(move || app::CasColorApp::new(config))
}

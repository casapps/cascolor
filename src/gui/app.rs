use iced::{
    widget::{button, column, container, row, scrollable, text, Row, Space},
    Alignment, Element, Length, Theme, Task,
};
use iced::widget::slider;

use crate::color::{CasColor, ColorFormat};
use crate::config::{Config, ThemeMode};

#[derive(Debug, Clone)]
pub enum Message {
    ColorChanged(CasColor),
    HueChanged(f32),
    SaturationChanged(f32),
    LightnessChanged(f32),
    CopyFormat(ColorFormat),
    LoadHistoryColor(String),
    ToggleAdvancedFormats,
    ActivateEyedropper,
    ThemeChanged,
}

pub struct CasColorApp {
    config: Config,
    current_color: CasColor,
    show_advanced_formats: bool,
    copy_feedback: Option<String>,
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl CasColorApp {
    pub fn new(config: Config) -> (Self, Task<Message>) {
        let initial_color = CasColor::from_rgb(128, 128, 200);
        let (h, s, l) = initial_color.to_hsl();
        
        (
            Self {
                config,
                current_color: initial_color,
                show_advanced_formats: false,
                copy_feedback: None,
                hue: h,
                saturation: s,
                lightness: l,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("cascolor - Beautiful Color Picker")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HueChanged(h) => {
                self.hue = h;
                self.current_color = CasColor::from_hsl(self.hue, self.saturation, self.lightness);
            }
            Message::SaturationChanged(s) => {
                self.saturation = s;
                self.current_color = CasColor::from_hsl(self.hue, self.saturation, self.lightness);
            }
            Message::LightnessChanged(l) => {
                self.lightness = l;
                self.current_color = CasColor::from_hsl(self.hue, self.saturation, self.lightness);
            }
            Message::CopyFormat(format) => {
                let text = self.format_color_string(format);
                if let Err(e) = crate::clipboard::copy_to_clipboard(&text) {
                    self.copy_feedback = Some(format!("Copy failed: {}", e));
                } else {
                    self.copy_feedback = Some(format!("Copied {} ✓", format));
                }
            }
            Message::LoadHistoryColor(hex) => {
                if let Ok(color) = CasColor::from_hex(&hex) {
                    self.current_color = color;
                    let (h, s, l) = color.to_hsl();
                    self.hue = h;
                    self.saturation = s;
                    self.lightness = l;
                }
            }
            Message::ToggleAdvancedFormats => {
                self.show_advanced_formats = !self.show_advanced_formats;
            }
            Message::ActivateEyedropper => {
                // Placeholder for eyedropper - will implement in next phase
                self.copy_feedback = Some("Eyedropper coming soon! 🎯".to_string());
            }
            Message::ThemeChanged => {
                self.config.general.theme = match self.config.general.theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                    _ => ThemeMode::Dark,
                };
            }
            _ => {}
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Large color preview
        let (r, g, b) = self.current_color.to_rgb();
        let color_preview = container(
            Space::new(Length::Fill, Length::Fixed(160.0))
        )
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(r, g, b))),
                border: iced::Border {
                    radius: 12.0.into(),
                    width: 2.0,
                    color: iced::Color::from_rgb8(200, 200, 210),
                },
                ..Default::default()
            }
        })
        .padding(20)
        .width(Length::Fill)
        .center_x(Length::Fill);

        // HEX display large
        let hex_text = text(self.current_color.to_hex())
            .size(28)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);

        // Eyedropper button (primary CTA)
        let eyedropper_button = button(
            text("🎯 Pick Color from Screen")
                .size(18)
                .align_x(iced::alignment::Horizontal::Center)
        )
        .on_press(Message::ActivateEyedropper)
        .padding(16)
        .width(Length::Fill);

        // HSL Sliders
        let hue_slider = column![
            text(format!("Hue: {:.0}°", self.hue)).size(13),
            slider(0.0..=360.0, self.hue, Message::HueChanged).step(1.0),
        ]
        .spacing(6)
        .width(Length::Fill);

        let saturation_slider = column![
            text(format!("Saturation: {:.0}%", self.saturation * 100.0)).size(13),
            slider(0.0..=1.0, self.saturation, Message::SaturationChanged).step(0.01),
        ]
        .spacing(6)
        .width(Length::Fill);

        let lightness_slider = column![
            text(format!("Lightness: {:.0}%", self.lightness * 100.0)).size(13),
            slider(0.0..=1.0, self.lightness, Message::LightnessChanged).step(0.01),
        ]
        .spacing(6)
        .width(Length::Fill);

        let sliders = column![hue_slider, saturation_slider, lightness_slider]
            .spacing(12)
            .padding(16);

        // Primary formats with copy buttons
        let primary_formats = column![
            self.format_row("HEX", self.current_color.to_hex(), ColorFormat::Hex),
            self.format_row("RGB", format!("rgb({}, {}, {})", r, g, b), ColorFormat::Rgb),
        ]
        .spacing(8)
        .padding(16);

        // Copy feedback
        let feedback = if let Some(ref msg) = self.copy_feedback {
            container(text(msg).size(13))
                .padding(8)
                .width(Length::Fill)
                .center_x(Length::Fill)
        } else {
            container(Space::new(Length::Fill, Length::Fixed(0.0)))
        };

        // Advanced formats (collapsible)
        let advanced_section = if self.show_advanced_formats {
            let (h, s, l) = self.current_color.to_hsl();
            let (h2, s2, v) = self.current_color.to_hsv();
            let (c, m, y, k) = self.current_color.to_cmyk();
            
            column![
                self.format_row("HSL", format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0), ColorFormat::Hsl),
                self.format_row("HSV", format!("hsv({:.0}, {:.0}%, {:.0}%)", h2, s2 * 100.0, v * 100.0), ColorFormat::Hsv),
                self.format_row("CMYK", format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c * 100.0, m * 100.0, y * 100.0, k * 100.0), ColorFormat::Cmyk),
            ]
            .spacing(8)
            .padding(16)
        } else {
            column![].spacing(0)
        };

        let toggle_advanced = button(
            text(if self.show_advanced_formats { "Show Less ⌃" } else { "More Formats ⌄" })
                .size(13)
        )
        .on_press(Message::ToggleAdvancedFormats)
        .padding(8)
        .width(Length::Fill);

        // Color history
        let history_title = text("Recent Colors").size(14).style(|theme: &Theme| {
            text::Style {
                color: Some(theme.palette().text),
            }
        });
        
        let mut history_row = Row::new().spacing(8).padding(16);
        for hex in self.config.color_history.iter().take(8) {
            if let Ok(color) = CasColor::from_hex(hex) {
                let (hr, hg, hb) = color.to_rgb();
                let hex_clone = hex.clone();
                let color_button = button(
                    Space::new(Length::Fixed(40.0), Length::Fixed(40.0))
                )
                .on_press(Message::LoadHistoryColor(hex_clone))
                .style(move |_theme: &Theme, status: button::Status| {
                    button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(hr, hg, hb))),
                        border: iced::Border {
                            radius: 20.0.into(),
                            width: if matches!(status, button::Status::Hovered) { 3.0 } else { 2.0 },
                            color: if matches!(status, button::Status::Hovered) {
                                iced::Color::from_rgb8(100, 150, 255)
                            } else {
                                iced::Color::from_rgb8(180, 180, 190)
                            },
                        },
                        ..Default::default()
                    }
                });
                history_row = history_row.push(color_button);
            }
        }

        // Main layout
        let content = scrollable(
            column![
                Space::new(Length::Fill, Length::Fixed(16.0)),
                color_preview,
                Space::new(Length::Fill, Length::Fixed(12.0)),
                hex_text,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                eyedropper_button,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                text("Adjust Color").size(16),
                sliders,
                Space::new(Length::Fill, Length::Fixed(8.0)),
                primary_formats,
                feedback,
                toggle_advanced,
                advanced_section,
                Space::new(Length::Fill, Length::Fixed(12.0)),
                history_title,
                history_row,
                Space::new(Length::Fill, Length::Fixed(20.0)),
            ]
            .spacing(0)
            .padding(16)
        );

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        self.get_iced_theme()
    }
}

impl CasColorApp {
    fn get_iced_theme(&self) -> Theme {
        match self.config.general.theme {
            ThemeMode::Dark => Theme::Dark,
            ThemeMode::Light => Theme::Light,
            _ => Theme::Dark,
        }
    }

    fn format_color_string(&self, format: ColorFormat) -> String {
        match format {
            ColorFormat::Hex => self.current_color.to_hex(),
            ColorFormat::Rgb => {
                let (r, g, b) = self.current_color.to_rgb();
                format!("rgb({}, {}, {})", r, g, b)
            }
            ColorFormat::Hsl => {
                let (h, s, l) = self.current_color.to_hsl();
                format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
            }
            ColorFormat::Hsv => {
                let (h, s, v) = self.current_color.to_hsv();
                format!("hsv({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, v * 100.0)
            }
            ColorFormat::Cmyk => {
                let (c, m, y, k) = self.current_color.to_cmyk();
                format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c * 100.0, m * 100.0, y * 100.0, k * 100.0)
            }
        }
    }

    fn format_row<'a>(&self, label: &'a str, value: String, format: ColorFormat) -> Element<'a, Message> {
        row![
            text(label).size(14).width(Length::Fixed(60.0)),
            text(value.clone()).size(14).width(Length::Fill),
            button(text("📋").size(14))
                .on_press(Message::CopyFormat(format))
                .padding(6)
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    }
}

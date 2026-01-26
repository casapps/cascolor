use iced::{
    widget::{button, column, container, row, text, canvas, Row, Space},
    Element, Length, Theme, Task, Point, Size, Color,
};
use iced::widget::slider;
use iced::mouse;

use crate::color::{CasColor, ColorFormat};
use crate::config::{Config, ThemeMode};

#[derive(Debug, Clone)]
pub enum Message {
    ColorChanged(CasColor),
    HueChanged(f32),
    SaturationChanged(f32),
    ValueChanged(f32),
    CanvasClicked(Point),
    CopyFormat(ColorFormat),
    LoadHistoryColor(String),
    ActivateEyedropper,
    ThemeChanged,
    SaveColor,
}

pub struct CasColorApp {
    config: Config,
    current_color: CasColor,
    copy_feedback: Option<String>,
    hue: f32,
    saturation: f32,
    value: f32, // Using HSV for better color picking UX
}

impl CasColorApp {
    pub fn new(config: Config) -> (Self, Task<Message>) {
        let initial_color = CasColor::from_rgb(255, 90, 90);
        let (h, s, v) = initial_color.to_hsv();
        
        (
            Self {
                config,
                current_color: initial_color,
                copy_feedback: None,
                hue: h,
                saturation: s,
                value: v,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("cascolor")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HueChanged(h) => {
                self.hue = h;
                self.update_color_from_hsv();
            }
            Message::SaturationChanged(s) => {
                self.saturation = s;
                self.update_color_from_hsv();
            }
            Message::ValueChanged(v) => {
                self.value = v;
                self.update_color_from_hsv();
            }
            Message::CanvasClicked(point) => {
                // 2D gradient picker: x = saturation, y = value (inverted)
                self.saturation = (point.x / 280.0).clamp(0.0, 1.0);
                self.value = (1.0 - point.y / 280.0).clamp(0.0, 1.0);
                self.update_color_from_hsv();
            }
            Message::CopyFormat(format) => {
                let text = self.format_color_string(format);
                if let Err(e) = crate::clipboard::copy_to_clipboard(&text) {
                    self.copy_feedback = Some(format!("Failed: {}", e));
                } else {
                    self.copy_feedback = Some(format!("Copied!"));
                }
            }
            Message::LoadHistoryColor(hex) => {
                if let Ok(color) = CasColor::from_hex(&hex) {
                    self.current_color = color;
                    let (h, s, v) = color.to_hsv();
                    self.hue = h;
                    self.saturation = s;
                    self.value = v;
                }
            }
            Message::SaveColor => {
                let hex = self.current_color.to_hex();
                if !self.config.color_history.contains(&hex) {
                    self.config.color_history.insert(0, hex);
                    if self.config.color_history.len() > 12 {
                        self.config.color_history.truncate(12);
                    }
                    let _ = self.config.save();
                }
                self.copy_feedback = Some("Saved!".to_string());
            }
            Message::ActivateEyedropper => {
                self.copy_feedback = Some("Coming soon!".to_string());
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
    
    fn update_color_from_hsv(&mut self) {
        self.current_color = CasColor::from_hsv(self.hue, self.saturation, self.value);
    }

    pub fn view(&self) -> Element<'_, Message> {
        let (r, g, b) = self.current_color.to_rgb();
        
        // Large color preview with rounded corners
        let color_preview = container(
            Space::new(Length::Fill, Length::Fixed(120.0))
        )
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(r, g, b))),
                border: iced::Border {
                    radius: 20.0.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba8(0, 0, 0, 0.15),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .padding(24);

        // 2D Gradient Picker (Saturation × Value)
        let gradient_picker = self.create_gradient_picker();

        // Hue slider with rainbow gradient
        let hue_slider = container(
            column![
                text(format!("{:.0}°", self.hue))
                    .size(13)
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
                slider(0.0..=360.0, self.hue, Message::HueChanged)
                    .step(1.0)
                    .width(Length::Fill),
            ]
            .spacing(4)
        )
        .padding([0, 24]);

        // Color values in compact grid
        let hex_val = self.current_color.to_hex();
        let values_grid = container(
            row![
                self.compact_value_box("HEX", hex_val.clone(), ColorFormat::Hex),
                self.compact_value_box("RGB", format!("{}, {}, {}", r, g, b), ColorFormat::Rgb),
            ]
            .spacing(8)
        )
        .padding([12, 24]);

        // Feedback message
        let feedback = if let Some(ref msg) = self.copy_feedback {
            container(
                text(msg)
                    .size(12)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .width(Length::Fill)
            .padding(24)
        } else {
            container(Space::new(Length::Fill, Length::Fixed(20.0)))
        };

        // Action buttons
        let actions = container(
            row![
                button(text("👁  Pick").align_x(iced::alignment::Horizontal::Center))
                    .on_press(Message::ActivateEyedropper)
                    .padding([10, 16])
                    .width(Length::Fill),
                button(text("💾 Save").align_x(iced::alignment::Horizontal::Center))
                    .on_press(Message::SaveColor)
                    .padding([10, 16])
                    .width(Length::Fill),
            ]
            .spacing(8)
        )
        .padding([0, 24]);

        // Color history swatches
        let mut history_row = Row::new().spacing(6);
        for hex in self.config.color_history.iter().take(12) {
            if let Ok(color) = CasColor::from_hex(hex) {
                let (hr, hg, hb) = color.to_rgb();
                let hex_clone = hex.clone();
                let swatch = button(
                    Space::new(Length::Fixed(32.0), Length::Fixed(32.0))
                )
                .on_press(Message::LoadHistoryColor(hex_clone))
                .style(move |_theme: &Theme, status: button::Status| {
                    button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(hr, hg, hb))),
                        border: iced::Border {
                            radius: 8.0.into(),
                            width: if matches!(status, button::Status::Hovered) { 2.0 } else { 0.0 },
                            color: iced::Color::from_rgb8(100, 150, 255),
                        },
                        shadow: if matches!(status, button::Status::Hovered) {
                            iced::Shadow {
                                color: iced::Color::from_rgba8(0, 0, 0, 0.2),
                                offset: iced::Vector::new(0.0, 2.0),
                                blur_radius: 8.0,
                            }
                        } else {
                            iced::Shadow::default()
                        },
                        ..Default::default()
                    }
                });
                history_row = history_row.push(swatch);
            }
        }

        let history = if !self.config.color_history.is_empty() {
            container(
                column![
                    text("Recently Used").size(12).style(|_theme: &Theme| {
                        text::Style {
                            color: Some(Color::from_rgb8(140, 140, 150)),
                        }
                    }),
                    Space::new(Length::Fill, Length::Fixed(6.0)),
                    history_row,
                ]
            )
            .padding(24)
        } else {
            container(Space::new(Length::Fill, Length::Fixed(0.0)))
        };

        // Main layout
        let content = column![
            Space::new(Length::Fill, Length::Fixed(20.0)),
            color_preview,
            gradient_picker,
            Space::new(Length::Fill, Length::Fixed(12.0)),
            hue_slider,
            Space::new(Length::Fill, Length::Fixed(16.0)),
            values_grid,
            feedback,
            Space::new(Length::Fill, Length::Fixed(8.0)),
            actions,
            Space::new(Length::Fill, Length::Fixed(16.0)),
            history,
            Space::new(Length::Fill, Length::Fixed(20.0)),
        ]
        .spacing(0);

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

    fn create_gradient_picker(&self) -> Element<'_, Message> {
        // Create a 280x280 gradient picker showing saturation×value for current hue
        let picker = canvas::Canvas::new(GradientPicker {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
        })
        .width(Length::Fixed(280.0))
        .height(Length::Fixed(280.0));

        container(picker)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }

    fn compact_value_box<'a>(&self, label: &'a str, value: String, format: ColorFormat) -> Element<'a, Message> {
        button(
            column![
                text(label).size(11).style(|_theme: &Theme| {
                    text::Style {
                        color: Some(Color::from_rgb8(140, 140, 150)),
                    }
                }),
                Space::new(Length::Fill, Length::Fixed(4.0)),
                text(value).size(14).style(|_theme: &Theme| {
                    text::Style {
                        color: Some(_theme.palette().text),
                    }
                }),
            ]
            .align_x(iced::alignment::Horizontal::Center)
        )
        .on_press(Message::CopyFormat(format))
        .padding(12)
        .width(Length::Fill)
        .style(|_theme: &Theme, status: button::Status| {
            button::Style {
                background: Some(iced::Background::Color(
                    if matches!(status, button::Status::Hovered) {
                        Color::from_rgb8(240, 240, 245)
                    } else {
                        Color::from_rgb8(250, 250, 252)
                    }
                )),
                border: iced::Border {
                    radius: 12.0.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                ..Default::default()
            }
        })
        .into()
    }
}

// 2D Gradient Picker Canvas
struct GradientPicker {
    hue: f32,
    saturation: f32,
    value: f32,
}

impl canvas::Program<Message> for GradientPicker {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Draw gradient (saturation left-right, value top-bottom)
        let cell_size = 4.0;
        for y in 0..(bounds.height / cell_size) as u32 {
            for x in 0..(bounds.width / cell_size) as u32 {
                let s = (x as f32 * cell_size) / bounds.width;
                let v = 1.0 - (y as f32 * cell_size) / bounds.height;
                
                let color = CasColor::from_hsv(self.hue, s, v);
                let (r, g, b) = color.to_rgb();
                
                let rect = canvas::Path::rectangle(
                    Point::new(x as f32 * cell_size, y as f32 * cell_size),
                    Size::new(cell_size, cell_size),
                );
                
                frame.fill(&rect, Color::from_rgb8(r, g, b));
            }
        }

        // Draw selection indicator
        let indicator_x = self.saturation * bounds.width;
        let indicator_y = (1.0 - self.value) * bounds.height;
        
        let outer_circle = canvas::Path::circle(
            Point::new(indicator_x, indicator_y),
            12.0,
        );
        let inner_circle = canvas::Path::circle(
            Point::new(indicator_x, indicator_y),
            10.0,
        );
        
        frame.stroke(
            &outer_circle,
            canvas::Stroke::default()
                .with_color(Color::WHITE)
                .with_width(3.0),
        );
        frame.stroke(
            &inner_circle,
            canvas::Stroke::default()
                .with_color(Color::from_rgb8(40, 40, 50))
                .with_width(2.0),
        );

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    return (canvas::event::Status::Captured, Some(Message::CanvasClicked(position)));
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::Backend, Terminal};
use std::io;
use std::time::Duration;

use crate::color::{CasColor, ColorFormat};
use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivePanel {
    Palette,     // Color palette grid (left)
    Gradient,    // Gradient picker (right top)
    FormatList,  // Format list (right bottom)
}

pub struct TuiApp {
    pub config: Config,
    pub current_color: CasColor,
    pub input_mode: InputMode,
    pub input_text: String,
    pub status_message: String,
    pub should_quit: bool,
    pub active_panel: ActivePanel,
    // Palette navigation (10 cols × 30 rows)
    pub palette_col: usize,
    pub palette_row: usize,
    // Gradient navigation (saturation × lightness)
    pub gradient_x: f32, // 0.0 to 1.0 (saturation)
    pub gradient_y: f32, // 0.0 to 1.0 (lightness/value)
    // Format list selection
    pub format_index: usize,
}

impl TuiApp {
    pub fn new(config: Config) -> Self {
        let current_color = CasColor::from_rgb(128, 128, 200);
        
        Self {
            config,
            current_color,
            input_mode: InputMode::Normal,
            input_text: String::new(),
            status_message: String::from("cascolor v0.1.0 | Tab: switch panel | h/j/k/l: navigate | c: copy | i: input | t: theme | q: quit"),
            should_quit: false,
            active_panel: ActivePanel::Gradient,
            palette_col: 5,
            palette_row: 15,
            gradient_x: 0.5,
            gradient_y: 0.5,
            format_index: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| super::ui::draw::<B>(f, self))?;

            if event::poll(Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            self.handle_key_event(key.code, key.modifiers);
                        }
                    }
                    Event::Mouse(mouse) => self.handle_mouse_event(mouse),
                    _ => {}
                }
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        if self.input_mode == InputMode::Editing {
            match key {
                KeyCode::Enter => {
                    // Try to parse input as color
                    if let Some(color) = self.parse_color_input(&self.input_text) {
                        self.current_color = color;
                        self.status_message = format!("Color set to: {}", self.input_text);
                    } else {
                        self.status_message = format!("Invalid color format: {}", self.input_text);
                    }
                    self.input_text.clear();
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    self.input_text.clear();
                    self.input_mode = InputMode::Normal;
                    self.status_message = "Input cancelled".to_string();
                }
                KeyCode::Backspace => {
                    self.input_text.pop();
                }
                KeyCode::Char(c) => {
                    self.input_text.push(c);
                }
                _ => {}
            }
            return;
        }

        // Normal mode key handling
        match key {
            KeyCode::Char('q') | KeyCode::Esc if modifiers.contains(KeyModifiers::NONE) => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Tab => {
                self.active_panel = match self.active_panel {
                    ActivePanel::Palette => ActivePanel::Gradient,
                    ActivePanel::Gradient => ActivePanel::FormatList,
                    ActivePanel::FormatList => ActivePanel::Palette,
                };
                self.status_message = format!("Switched to: {:?} panel", self.active_panel);
            }
            KeyCode::Char('i') => {
                self.input_mode = InputMode::Editing;
                self.status_message = "Enter color (HEX, RGB, HSL, etc.) | Enter: apply | Esc: cancel".to_string();
            }
            KeyCode::Char('t') => {
                self.toggle_theme();
            }
            KeyCode::Char('c') => {
                self.copy_current_format();
            }
            // Vim navigation
            KeyCode::Char('h') | KeyCode::Left => self.move_left(),
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('l') | KeyCode::Right => self.move_right(),
            // Quick copy shortcuts
            KeyCode::Char('1') => self.copy_format(ColorFormat::Hex),
            KeyCode::Char('2') => self.copy_format(ColorFormat::Rgb),
            KeyCode::Char('3') => self.copy_format(ColorFormat::Hsl),
            KeyCode::Char('4') => self.copy_format(ColorFormat::Hsv),
            KeyCode::Char('5') => self.copy_format(ColorFormat::Cmyk),
            _ => {}
        }
    }

    fn move_left(&mut self) {
        match self.active_panel {
            ActivePanel::Palette => {
                if self.palette_col > 0 {
                    self.palette_col -= 1;
                    self.update_color_from_palette();
                }
            }
            ActivePanel::Gradient => {
                self.gradient_x = (self.gradient_x - 0.05).max(0.0);
                self.update_color_from_gradient();
            }
            ActivePanel::FormatList => {}
        }
    }

    fn move_right(&mut self) {
        match self.active_panel {
            ActivePanel::Palette => {
                if self.palette_col < 9 {
                    self.palette_col += 1;
                    self.update_color_from_palette();
                }
            }
            ActivePanel::Gradient => {
                self.gradient_x = (self.gradient_x + 0.05).min(1.0);
                self.update_color_from_gradient();
            }
            ActivePanel::FormatList => {}
        }
    }

    fn move_up(&mut self) {
        match self.active_panel {
            ActivePanel::Palette => {
                if self.palette_row > 0 {
                    self.palette_row -= 1;
                    self.update_color_from_palette();
                }
            }
            ActivePanel::Gradient => {
                self.gradient_y = (self.gradient_y - 0.05).max(0.0);
                self.update_color_from_gradient();
            }
            ActivePanel::FormatList => {
                if self.format_index > 0 {
                    self.format_index -= 1;
                }
            }
        }
    }

    fn move_down(&mut self) {
        match self.active_panel {
            ActivePanel::Palette => {
                if self.palette_row < 29 {
                    self.palette_row += 1;
                    self.update_color_from_palette();
                }
            }
            ActivePanel::Gradient => {
                self.gradient_y = (self.gradient_y + 0.05).min(1.0);
                self.update_color_from_gradient();
            }
            ActivePanel::FormatList => {
                if self.format_index < 4 {
                    self.format_index += 1;
                }
            }
        }
    }

    fn update_color_from_palette(&mut self) {
        // Generate color from palette position
        let hue = (self.palette_row as f32 / 30.0) * 360.0;
        let lightness = 0.2 + (self.palette_col as f32 / 10.0) * 0.6;
        let saturation = 0.8;
        
        self.current_color = CasColor::from_hsl(hue, saturation, lightness);
        self.status_message = format!("Palette: col={} row={}", self.palette_col, self.palette_row);
    }

    fn update_color_from_gradient(&mut self) {
        // Get current hue from color
        let (h, _, _) = self.current_color.to_hsl();
        
        // Update with new saturation and lightness
        let saturation = self.gradient_x;
        let lightness = 1.0 - self.gradient_y; // Invert Y for natural feel
        
        self.current_color = CasColor::from_hsl(h, saturation, lightness);
        self.status_message = format!("Gradient: sat={:.2} light={:.2}", saturation, lightness);
    }

    fn handle_mouse_event(&mut self, _mouse: MouseEvent) {
        // Mouse support can be added later
    }

    fn toggle_theme(&mut self) {
        use crate::config::ThemeMode;
        self.config.general.theme = match self.config.general.theme {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
            _ => ThemeMode::Dark,
        };
        self.status_message = format!("Theme: {:?}", self.config.general.theme);
    }

    fn copy_current_format(&mut self) {
        let formats = [
            ColorFormat::Hex,
            ColorFormat::Rgb,
            ColorFormat::Hsl,
            ColorFormat::Hsv,
            ColorFormat::Cmyk,
        ];
        
        if let Some(format) = formats.get(self.format_index) {
            self.copy_format(*format);
        }
    }

    fn copy_format(&mut self, format: ColorFormat) {
        let text = match format {
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
        };

        if let Err(e) = crate::clipboard::copy_to_clipboard(&text) {
            self.status_message = format!("Copy failed: {}", e);
        } else {
            self.status_message = format!("Copied {} to clipboard: {}", format, text);
        }
    }

    fn parse_color_input(&self, input: &str) -> Option<CasColor> {
        let input = input.trim();
        
        // Try HEX
        if input.starts_with('#') && (input.len() == 7 || input.len() == 4) {
            return CasColor::from_hex(input).ok();
        }
        
        // Try RGB
        if input.starts_with("rgb(") && input.ends_with(')') {
            let inner = &input[4..input.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].parse::<u8>(),
                    parts[1].parse::<u8>(),
                    parts[2].parse::<u8>(),
                ) {
                    return Some(CasColor::from_rgb(r, g, b));
                }
            }
        }
        
        // Try HSL
        if input.starts_with("hsl(") && input.ends_with(')') {
            let inner = &input[4..input.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim().trim_end_matches('%')).collect();
            if parts.len() == 3 {
                if let (Ok(h), Ok(s), Ok(l)) = (
                    parts[0].parse::<f32>(),
                    parts[1].parse::<f32>(),
                    parts[2].parse::<f32>(),
                ) {
                    return Some(CasColor::from_hsl(h, s / 100.0, l / 100.0));
                }
            }
        }
        
        None
    }
}

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app::{ActivePanel, InputMode, TuiApp};
use crate::config::ThemeMode;

pub fn draw<B: Backend>(f: &mut Frame, app: &TuiApp) {
    let is_dark = matches!(app.config.general.theme, ThemeMode::Dark);
    
    let (bg, fg, border, accent) = if is_dark {
        (Color::Rgb(30, 30, 40), Color::Rgb(220, 220, 230), Color::Rgb(80, 80, 100), Color::Rgb(100, 150, 255))
    } else {
        (Color::Rgb(250, 250, 255), Color::Rgb(30, 30, 40), Color::Rgb(180, 180, 200), Color::Rgb(60, 100, 200))
    };

    // Main layout: [Palette 30%][Gradient+Formats 70%]
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    // Left: Color Palette Grid
    draw_palette_grid::<B>(f, app, main_chunks[0], bg, fg, border, accent);

    // Right side: [Gradient][Formats][Status]
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Gradient picker
            Constraint::Min(15),         // Format list
            Constraint::Length(3),       // Status bar
        ])
        .split(main_chunks[1]);

    // Gradient picker (2D selector)
    draw_gradient_picker::<B>(f, app, right_chunks[0], bg, fg, border, accent);

    // Format list with copy buttons
    draw_format_list::<B>(f, app, right_chunks[1], bg, fg, border, accent);

    // Status bar at bottom
    draw_status_bar::<B>(f, app, right_chunks[2], bg, fg, border);

    // Input overlay if in edit mode
    if app.input_mode == InputMode::Editing {
        draw_input_overlay::<B>(f, app, bg, fg, border, accent);
    }
}

fn draw_palette_grid<B: Backend>(
    f: &mut Frame,
    app: &TuiApp,
    area: Rect,
    bg: Color,
    _fg: Color,
    border: Color,
    accent: Color,
) {
    let is_active = app.active_panel == ActivePanel::Palette;
    let border_style = if is_active {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border)
    };

    let block = Block::default()
        .title(" Color Palette ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Draw 10x30 color grid
    let cell_width = (inner.width.saturating_sub(2)) / 10;
    let cell_height = 1;
    
    for row in 0..30.min(inner.height as usize) {
        for col in 0..10 {
            let x = inner.x + (col as u16 * cell_width);
            let y = inner.y + row as u16;
            
            if x >= inner.x + inner.width || y >= inner.y + inner.height {
                break;
            }

            // Generate color for this palette cell
            let hue = (row as f32 / 30.0) * 360.0;
            let lightness = 0.2 + (col as f32 / 10.0) * 0.6;
            let saturation = 0.8;
            
            let color = crate::color::CasColor::from_hsl(hue, saturation, lightness);
            let (r, g, b) = color.to_rgb();
            let cell_color = Color::Rgb(r, g, b);

            // Highlight selected cell
            let is_selected = app.active_panel == ActivePanel::Palette && app.palette_col == col && app.palette_row == row;
            let symbol = if is_selected { "█" } else { "▓" };
            
            let cell_area = Rect::new(x, y, cell_width.min(3), cell_height);
            let text = if is_selected {
                Span::styled(symbol.repeat(cell_width as usize), Style::default().fg(cell_color).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(symbol.repeat(cell_width as usize), Style::default().fg(cell_color))
            };
            
            f.render_widget(Paragraph::new(text), cell_area);
        }
    }
}

fn draw_gradient_picker<B: Backend>(
    f: &mut Frame,
    app: &TuiApp,
    area: Rect,
    bg: Color,
    _fg: Color,
    border: Color,
    accent: Color,
) {
    let is_active = app.active_panel == ActivePanel::Gradient;
    let border_style = if is_active {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border)
    };

    let block = Block::default()
        .title(" Gradient Picker ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get current hue
    let (base_hue, _, _) = app.current_color.to_hsl();

    // Draw 2D gradient (saturation × lightness)
    for y in 0..inner.height {
        for x in 0..inner.width {
            let saturation = x as f32 / inner.width as f32;
            let lightness = 1.0 - (y as f32 / inner.height as f32);
            
            let color = crate::color::CasColor::from_hsl(base_hue, saturation, lightness);
            let (r, g, b) = color.to_rgb();
            let cell_color = Color::Rgb(r, g, b);

            // Check if this is cursor position
            let cursor_x = (app.gradient_x * inner.width as f32) as u16;
            let cursor_y = ((1.0 - app.gradient_y) * inner.height as f32) as u16;
            let is_cursor = is_active && x == cursor_x && y == cursor_y;

            let symbol = if is_cursor { "●" } else { "█" };
            let style = if is_cursor {
                Style::default().fg(Color::White).bg(cell_color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(cell_color)
            };

            let cell_area = Rect::new(inner.x + x, inner.y + y, 1, 1);
            f.render_widget(Paragraph::new(Span::styled(symbol, style)), cell_area);
        }
    }
}

fn draw_format_list<B: Backend>(
    f: &mut Frame,
    app: &TuiApp,
    area: Rect,
    bg: Color,
    fg: Color,
    border: Color,
    accent: Color,
) {
    let is_active = app.active_panel == ActivePanel::FormatList;
    let border_style = if is_active {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border)
    };

    let block = Block::default()
        .title(" Color Formats (c: copy | 1-5: quick copy) ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(bg).fg(fg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Current color preview
    let (r, g, b) = app.current_color.to_rgb();
    let color_rgb = Color::Rgb(r, g, b);
    
    let mut lines = vec![
        Line::from(vec![
            Span::styled("  ████████  ", Style::default().fg(color_rgb).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  Current Color"), Style::default().fg(fg)),
        ]),
        Line::from(""),
    ];

    // Format list
    let formats = vec![
        ("1", "HEX", app.current_color.to_hex()),
        ("2", "RGB", {
            let (r, g, b) = app.current_color.to_rgb();
            format!("rgb({}, {}, {})", r, g, b)
        }),
        ("3", "HSL", {
            let (h, s, l) = app.current_color.to_hsl();
            format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
        }),
        ("4", "HSV", {
            let (h, s, v) = app.current_color.to_hsv();
            format!("hsv({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, v * 100.0)
        }),
        ("5", "CMYK", {
            let (c, m, y, k) = app.current_color.to_cmyk();
            format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c * 100.0, m * 100.0, y * 100.0, k * 100.0)
        }),
    ];

    for (idx, (key, name, value)) in formats.iter().enumerate() {
        let is_selected = is_active && app.format_index == idx;
        let style = if is_selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(fg)
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" [{}] ", key), Style::default().fg(accent)),
            Span::styled(format!("{:5} ", name), style),
            Span::styled(value, style),
        ]));
    }

    let para = Paragraph::new(lines)
        .style(Style::default().bg(bg))
        .alignment(Alignment::Left);
    
    f.render_widget(para, inner);
}

fn draw_status_bar<B: Backend>(
    f: &mut Frame,
    app: &TuiApp,
    area: Rect,
    bg: Color,
    fg: Color,
    _border: Color,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(_border))
        .style(Style::default().bg(bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(fg).bg(bg))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    f.render_widget(status, inner);
}

fn draw_input_overlay<B: Backend>(
    f: &mut Frame,
    app: &TuiApp,
    bg: Color,
    fg: Color,
    _border: Color,
    accent: Color,
) {
    // Center overlay
    let area = centered_rect(60, 20, f.area());

    let block = Block::default()
        .title(" Enter Color ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Input: ", Style::default().fg(fg)),
            Span::styled(&app.input_text, Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled("_", Style::default().fg(accent)),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Examples:", Style::default().fg(fg).add_modifier(Modifier::DIM))),
        Line::from(Span::styled("    #FF5733", Style::default().fg(fg).add_modifier(Modifier::DIM))),
        Line::from(Span::styled("    rgb(255, 87, 51)", Style::default().fg(fg).add_modifier(Modifier::DIM))),
        Line::from(Span::styled("    hsl(9, 100%, 60%)", Style::default().fg(fg).add_modifier(Modifier::DIM))),
    ];

    let para = Paragraph::new(text).alignment(Alignment::Left);
    f.render_widget(para, inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

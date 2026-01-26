use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, DrawingArea, Grid, Label,
    Orientation, Scale,
};
use libadwaita::StyleManager;

use crate::color::CasColor;
use crate::config::Config;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const APP_ID: &str = "com.casapps.cascolor";

pub struct ColorPickerApp {
    config: Arc<RefCell<Config>>,
    current_color: Rc<RefCell<CasColor>>,
}

impl ColorPickerApp {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(RefCell::new(config)),
            current_color: Rc::new(RefCell::new(CasColor::from_rgb(255, 90, 90))),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize GTK
        let app = Application::builder().application_id(APP_ID).build();

        let config = self.config.clone();
        let current_color = self.current_color.clone();

        app.connect_activate(move |app| {
            build_ui(app, config.clone(), current_color.clone());
        });

        app.run();
        Ok(())
    }
}

fn build_ui(
    app: &Application,
    _config: Arc<RefCell<Config>>,
    current_color: Rc<RefCell<CasColor>>,
) {
    // Enable Adwaita styling
    StyleManager::default().set_color_scheme(libadwaita::ColorScheme::PreferDark);

    // Main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("cascolor")
        .default_width(360)
        .default_height(680)
        .build();

    // Main container
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.set_margin_top(24);
    main_box.set_margin_bottom(24);
    main_box.set_margin_start(24);
    main_box.set_margin_end(24);
    main_box.set_spacing(16);

    // Large color preview
    let preview = DrawingArea::new();
    preview.set_content_width(312);
    preview.set_content_height(120);
    preview.set_halign(gtk4::Align::Fill);
    preview.add_css_class("color-preview");

    let preview_color = current_color.clone();
    preview.set_draw_func(move |_, cr, width, height| {
        let color = preview_color.borrow();
        let (r, g, b) = color.to_rgb();

        // Draw rounded rectangle
        let radius = 20.0;
        cr.new_sub_path();
        cr.arc(
            radius,
            radius,
            radius,
            std::f64::consts::PI,
            3.0 * std::f64::consts::PI / 2.0,
        );
        cr.arc(
            width as f64 - radius,
            radius,
            radius,
            3.0 * std::f64::consts::PI / 2.0,
            0.0,
        );
        cr.arc(
            width as f64 - radius,
            height as f64 - radius,
            radius,
            0.0,
            std::f64::consts::PI / 2.0,
        );
        cr.arc(
            radius,
            height as f64 - radius,
            radius,
            std::f64::consts::PI / 2.0,
            std::f64::consts::PI,
        );
        cr.close_path();

        cr.set_source_rgb(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
        let _ = cr.fill();
    });

    // 2D Gradient picker
    let gradient = DrawingArea::new();
    gradient.set_content_width(280);
    gradient.set_content_height(280);
    gradient.set_halign(gtk4::Align::Center);

    let hue = Rc::new(RefCell::new(0.0f32));
    let saturation = Rc::new(RefCell::new(1.0f32));
    let value = Rc::new(RefCell::new(1.0f32));

    let gradient_hue = hue.clone();
    let gradient_sat = saturation.clone();
    let gradient_val = value.clone();

    gradient.set_draw_func(move |_, cr, width, height| {
        let h = *gradient_hue.borrow();
        let s = *gradient_sat.borrow();
        let v = *gradient_val.borrow();

        // Draw gradient grid
        let cell_size = 4.0f64;
        for y in 0..(height / cell_size as i32) {
            for x in 0..(width / cell_size as i32) {
                let sat = (x as f32 * cell_size as f32) / width as f32;
                let val = 1.0 - (y as f32 * cell_size as f32) / height as f32;

                let color = CasColor::from_hsv(h, sat, val);
                let (r, g, b) = color.to_rgb();

                cr.rectangle(
                    x as f64 * cell_size,
                    y as f64 * cell_size,
                    cell_size,
                    cell_size,
                );
                cr.set_source_rgb(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
                let _ = cr.fill();
            }
        }

        // Draw selection indicator
        let ind_x = s * width as f32;
        let ind_y = (1.0 - v) * height as f32;

        cr.arc(ind_x as f64, ind_y as f64, 12.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(3.0);
        let _ = cr.stroke();

        cr.arc(ind_x as f64, ind_y as f64, 10.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(0.15, 0.15, 0.2);
        cr.set_line_width(2.0);
        let _ = cr.stroke();
    });

    // Add click handler for gradient
    let gesture = gtk4::GestureClick::new();
    let click_color = current_color.clone();
    let click_hue = hue.clone();
    let click_sat = saturation.clone();
    let click_val = value.clone();
    let click_gradient = gradient.clone();
    let click_preview = preview.clone();

    gesture.connect_pressed(move |_, _, x, y| {
        let width = click_gradient.content_width() as f32;
        let height = click_gradient.content_height() as f32;

        let new_sat = (x as f32 / width).clamp(0.0, 1.0);
        let new_val = (1.0 - y as f32 / height).clamp(0.0, 1.0);

        *click_sat.borrow_mut() = new_sat;
        *click_val.borrow_mut() = new_val;

        let h = *click_hue.borrow();
        let new_color = CasColor::from_hsv(h, new_sat, new_val);
        *click_color.borrow_mut() = new_color;

        click_gradient.queue_draw();
        click_preview.queue_draw();
    });
    gradient.add_controller(gesture);

    // Hue slider
    let hue_scale = Scale::with_range(Orientation::Horizontal, 0.0, 360.0, 1.0);
    hue_scale.set_value(0.0);
    hue_scale.set_draw_value(true);
    hue_scale.set_value_pos(gtk4::PositionType::Bottom);

    let hue_color = current_color.clone();
    let hue_hue = hue.clone();
    let hue_sat = saturation.clone();
    let hue_val = value.clone();
    let hue_gradient = gradient.clone();
    let hue_preview = preview.clone();

    hue_scale.connect_value_changed(move |scale| {
        let new_hue = scale.value() as f32;
        *hue_hue.borrow_mut() = new_hue;

        let s = *hue_sat.borrow();
        let v = *hue_val.borrow();
        let new_color = CasColor::from_hsv(new_hue, s, v);
        *hue_color.borrow_mut() = new_color;

        hue_gradient.queue_draw();
        hue_preview.queue_draw();
    });

    let hue_label = Label::new(Some("Hue"));
    hue_label.set_halign(gtk4::Align::Start);

    // HEX/RGB display
    let values_grid = Grid::new();
    values_grid.set_column_spacing(8);
    values_grid.set_row_spacing(8);
    values_grid.set_halign(gtk4::Align::Fill);

    let hex_button = Button::with_label("HEX: #FF5A5A");
    hex_button.set_hexpand(true);
    let rgb_button = Button::with_label("RGB: 255, 90, 90");
    rgb_button.set_hexpand(true);

    values_grid.attach(&hex_button, 0, 0, 1, 1);
    values_grid.attach(&rgb_button, 1, 0, 1, 1);

    // Action buttons
    let actions_box = GtkBox::new(Orientation::Horizontal, 8);
    let pick_button = Button::with_label("👁 Pick");
    pick_button.set_hexpand(true);
    pick_button.add_css_class("suggested-action");

    let save_button = Button::with_label("💾 Save");
    save_button.set_hexpand(true);

    actions_box.append(&pick_button);
    actions_box.append(&save_button);

    // Color history
    let history_label = Label::new(Some("Recently Used"));
    history_label.set_halign(gtk4::Align::Start);
    history_label.add_css_class("dim-label");

    let history_box = GtkBox::new(Orientation::Horizontal, 6);
    history_box.set_halign(gtk4::Align::Start);

    // Add placeholder swatches
    for _ in 0..8 {
        let swatch = Button::new();
        swatch.set_size_request(32, 32);
        swatch.add_css_class("circular");
        history_box.append(&swatch);
    }

    // Assemble UI
    main_box.append(&preview);
    main_box.append(&gradient);
    main_box.append(&hue_label);
    main_box.append(&hue_scale);
    main_box.append(&values_grid);
    main_box.append(&actions_box);
    main_box.append(&history_label);
    main_box.append(&history_box);

    window.set_child(Some(&main_box));
    window.present();
}

mod app_gtk;
mod eyedropper;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = app_gtk::ColorPickerApp::new(config);
    app.run()
}

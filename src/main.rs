// main.rs
mod gui;
mod adb;
mod utils;
mod config;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Universal Android Debloater",
        options,
        Box::new(|cc| Box::new(gui::DebloaterApp::new(cc))),
    )
}
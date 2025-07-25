// src/main.rs
mod gui;
mod adb;
mod utils;
mod config;

use eframe::NativeOptions;
use eframe::egui::IconData;

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../assets/img/logo.png"))
            .expect("Failed to load icon.png")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_icon(load_icon()), // This sets the icon
        ..Default::default()
    };
    eframe::run_native(
        "P.U.R.G.E. - Package Uninstaller and Resource & Garbage Eliminator",
        options,
        Box::new(|cc| Box::new(gui::DebloaterApp::new(cc))),
    )
}
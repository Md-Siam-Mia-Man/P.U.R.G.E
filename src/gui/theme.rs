// src/gui/theme.rs
use eframe::egui;
use egui::{style::Spacing, FontData, FontDefinitions, FontFamily, Style, TextStyle, Visuals};

pub struct Theme {
    pub background: egui::Color32,
    pub title_bar: egui::Color32,
    pub surface: egui::Color32,
    pub primary: egui::Color32,
    pub primary_variant: egui::Color32,
    pub danger: egui::Color32,
    pub on_primary: egui::Color32,
    pub on_surface: egui::Color32,
    pub on_surface_variant: egui::Color32,

    pub status_ok: egui::Color32,
    pub status_warn: egui::Color32,
    pub status_err: egui::Color32,
    pub status_neutral: egui::Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: egui::Color32::from_rgb(26, 26, 28),
            title_bar: egui::Color32::from_rgb(18, 18, 19),
            surface: egui::Color32::from_rgb(37, 37, 40),
            primary: egui::Color32::from_rgb(255, 11, 85),
            primary_variant: egui::Color32::from_rgb(207, 9, 69),
            danger: egui::Color32::from_rgb(255, 30, 90),
            on_primary: egui::Color32::from_rgb(240, 240, 240),
            on_surface: egui::Color32::from_rgb(241, 241, 241),
            on_surface_variant: egui::Color32::from_rgb(170, 170, 170),

            status_ok: egui::Color32::from_rgb(30, 200, 150),
            status_warn: egui::Color32::from_rgb(255, 180, 0),
            status_err: egui::Color32::from_rgb(255, 80, 80),
            status_neutral: egui::Color32::from_rgb(150, 150, 150),
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, theme: &Theme) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Poppins".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/Poppins.ttf")),
    );
    fonts
        .families
        .values_mut()
        .for_each(|family| family.insert(0, "Poppins".to_owned()));
    ctx.set_fonts(fonts);

    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(theme.on_surface);
    visuals.window_fill = theme.background;
    visuals.panel_fill = theme.background;
    visuals.widgets.noninteractive.bg_fill = theme.surface;
    visuals.widgets.noninteractive.fg_stroke.color = theme.on_surface_variant;
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
    visuals.widgets.inactive.bg_fill = theme.surface;
    visuals.widgets.inactive.fg_stroke.color = theme.on_surface;
    visuals.widgets.hovered.bg_fill = theme.primary_variant;
    visuals.widgets.hovered.fg_stroke.color = theme.on_primary;
    visuals.widgets.active.bg_fill = theme.primary;
    visuals.widgets.active.fg_stroke.color = theme.on_primary;
    visuals.selection.bg_fill = theme.primary;
    visuals.selection.stroke.color = theme.on_primary;

    let mut style = Style {
        text_styles: [
            (
                TextStyle::Heading,
                egui::FontId::new(24.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                egui::FontId::new(16.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                egui::FontId::new(15.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                egui::FontId::new(16.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                egui::FontId::new(13.0, FontFamily::Proportional),
            ),
        ]
        .into(),
        spacing: Spacing {
            item_spacing: egui::vec2(12.0, 12.0),
            button_padding: egui::vec2(12.0, 8.0),
            interact_size: egui::vec2(40.0, 40.0),
            ..Default::default()
        },
        visuals: visuals.clone(),
        ..Default::default()
    };

    style.interaction.selectable_labels = false;
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
    style.visuals.window_rounding = egui::Rounding::ZERO;

    ctx.set_style(style);
}

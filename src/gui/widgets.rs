// src/gui/widgets.rs
use crate::config::PackageInfo;
use crate::gui::theme::Theme;
use eframe::egui;

pub fn package_card(
    ui: &mut egui::Ui,
    theme: &Theme,
    info: &PackageInfo,
    is_selected: bool,
    is_active: bool,
) {
    let card_color = if is_selected {
        theme.primary_variant
    } else {
        theme.surface
    };
    let stroke = if is_active {
        egui::Stroke::new(2.0, theme.primary)
    } else {
        egui::Stroke::NONE
    };

    egui::Frame::none()
        .inner_margin(egui::Margin::same(12.0))
        .rounding(ui.style().visuals.widgets.noninteractive.rounding)
        .fill(card_color)
        .stroke(stroke)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                let visuals = ui.style().visuals.widgets.inactive;
                ui.painter()
                    .rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);
                if is_selected {
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "✔",
                        egui::FontId::proportional(16.0),
                        theme.on_primary,
                    );
                }

                let status_color = match info.removal.as_deref() {
                    Some("Recommended") => theme.status_ok,
                    Some("Advanced") => theme.status_warn,
                    Some("Expert") => theme.status_err,
                    _ => theme.status_neutral,
                };
                let (dot_rect, _) =
                    ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter()
                    .circle_filled(dot_rect.center(), 4.0, status_color);

                ui.add_space(4.0);

                // This vertical layout will now expand to fill the rest of the horizontal space
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&info.id).size(17.0).strong());
                    if let Some(desc) = &info.description {
                        ui.add_space(2.0);
                        let truncated = desc.lines().next().unwrap_or("").to_string();
                        ui.label(
                            egui::RichText::new(truncated)
                                .size(14.0)
                                .color(theme.on_surface_variant),
                        );
                    }
                });
            });
        });
}

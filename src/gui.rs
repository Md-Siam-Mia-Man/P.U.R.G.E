// gui.rs
use eframe::egui;
use crate::{adb, config::{PackageInfo, load_uad_list}};
use std::collections::{HashSet, BTreeSet};

pub struct DebloaterApp {
    uad_packages: Vec<PackageInfo>,
    installed_packages: HashSet<String>,
    selected: HashSet<String>,
    adb_output: String,
    search_query: String,
    all_lists: Vec<String>,
    all_removals: Vec<String>,
    filter_list: String,
    filter_removal: String,
}

impl DebloaterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_theme(&cc.egui_ctx);
        
        let uad_packages = load_uad_list();
        
        let all_lists_set: BTreeSet<String> = uad_packages.iter()
            .filter_map(|p| p.list.clone())
            .collect();
        let mut all_lists: Vec<String> = all_lists_set.into_iter().collect();
        all_lists.insert(0, "All".to_string());
        
        let all_removals_set: BTreeSet<String> = uad_packages.iter()
            .filter_map(|p| p.removal.clone())
            .collect();
        let mut all_removals: Vec<String> = all_removals_set.into_iter().collect();
        all_removals.insert(0, "All".to_string());
        
        Self {
            uad_packages,
            installed_packages: HashSet::new(),
            selected: HashSet::new(),
            adb_output: "Welcome to UAD!\n1. Connect your device with USB Debugging enabled.\n2. Click 'Detect Device'.\n3. Click 'List Packages' to begin.".to_string(),
            search_query: String::new(),
            all_lists,
            all_removals,
            filter_list: "All".to_string(),
            filter_removal: "All".to_string(),
        }
    }
}

fn setup_theme(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Poppins".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Poppins.ttf")),
    );

    fonts
        .families
        .values_mut()
        .for_each(|family| family.insert(0, "Poppins".to_owned()));

    ctx.set_fonts(fonts);

    let black = egui::Color32::from_hex("#0f0f0f").unwrap();
    let dark_grey = egui::Color32::from_hex("#272727").unwrap();
    let mid_grey = egui::Color32::from_hex("#404040").unwrap();
    let light_grey = egui::Color32::from_hex("#aaaaaa").unwrap();
    let white = egui::Color32::from_hex("#f1f1f1").unwrap();
    let red = egui::Color32::from_hex("#ff0b55").unwrap();
    let red_dark = egui::Color32::from_hex("#cf0f47").unwrap();

    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = black;
    visuals.panel_fill = black;
    visuals.override_text_color = Some(white);
    
    visuals.widgets.noninteractive.bg_fill = dark_grey;
    visuals.widgets.noninteractive.fg_stroke.color = light_grey; 
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    
    visuals.widgets.inactive.bg_fill = dark_grey;
    visuals.widgets.inactive.fg_stroke.color = white;
    
    visuals.widgets.hovered.bg_fill = mid_grey;
    visuals.widgets.hovered.fg_stroke.color = white;

    visuals.widgets.active.bg_fill = red;
    visuals.widgets.active.fg_stroke.color = white;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, red_dark);
    
    visuals.selection.bg_fill = red_dark;
    
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(22.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(15.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(13.0, egui::FontFamily::Proportional)),
    ].into();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.interact_size.y = 32.0;
    style.visuals.window_rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);

    ctx.set_style(style);
}

impl eframe::App for DebloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let filtered_packages: Vec<&PackageInfo> = self.uad_packages.iter()
            .filter(|info| {
                let is_installed = self.installed_packages.contains(&info.id);
                let matches_search = self.search_query.is_empty() || info.id.to_lowercase().contains(&self.search_query.to_lowercase());
                let list_filter_pass = self.filter_list == "All" || info.list.as_deref() == Some(&self.filter_list);
                let removal_filter_pass = self.filter_removal == "All" || info.removal.as_deref() == Some(&self.filter_removal);
                is_installed && matches_search && list_filter_pass && removal_filter_pass
            })
            .collect();
            
        egui::SidePanel::left("control_panel").width_range(280.0..=450.0).show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.add_space(10.0);
                ui.heading("Universal Android Debloater");
                ui.add_space(20.0);

                ui.label("Device Connection");
                ui.horizontal(|ui| {
                    if ui.button("Detect Device").clicked() { self.adb_output = adb::detect_device(); }
                    if ui.button("List Packages").clicked() {
                        let installed = adb::list_packages();
                        if !installed.is_empty() && installed[0].contains("Failed") {
                             self.adb_output = installed[0].clone();
                        } else {
                            self.installed_packages = installed.into_iter().collect();
                            self.selected.clear();
                            self.adb_output = format!("Found {} installed packages.", self.installed_packages.len());
                        }
                    }
                });

                ui.add_space(20.0);
                ui.label("Package Selection");
                let uninstall_text = format!("Uninstall ({}) Selected", self.selected.len());
                let uninstall_button = egui::Button::new(egui::RichText::new(uninstall_text).color(ctx.style().visuals.widgets.active.fg_stroke.color));
                if ui.add_enabled(!self.selected.is_empty(), uninstall_button).clicked() {
                    let selected_clone = self.selected.clone();
                    for pkg in &selected_clone { adb::uninstall(pkg); }
                    self.adb_output = format!("Uninstalled {} packages. Refreshing list...", selected_clone.len());
                    let installed = adb::list_packages();
                    self.installed_packages = installed.into_iter().collect();
                    self.selected.clear();
                    self.adb_output.push_str("\nList refreshed.");
                }
                
                ui.horizontal(|ui| {
                    if ui.button("Select All").clicked() {
                        for pkg in &filtered_packages {
                            self.selected.insert(pkg.id.clone());
                        }
                    }
                    if ui.button("Deselect All").clicked() { self.selected.clear(); }
                    if ui.button("Reboot Device").clicked() {
                        adb::reboot_device();
                        self.adb_output = "Reboot command sent to device.".to_string();
                    }
                });

                ui.add_space(20.0);
                ui.label("Search & Filter");
                ui.text_edit_singleline(&mut self.search_query).on_hover_text("Search by package name");

                egui::ComboBox::from_id_source("list_filter").selected_text(self.filter_list.clone()).show_ui(ui, |ui| {
                    for list_name in &self.all_lists {
                        ui.selectable_value(&mut self.filter_list, list_name.clone(), list_name.clone());
                    }
                });

                egui::ComboBox::from_id_source("removal_filter").selected_text(self.filter_removal.clone()).show_ui(ui, |ui| {
                    for removal_name in &self.all_removals {
                        ui.selectable_value(&mut self.filter_removal, removal_name.clone(), removal_name.clone());
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_log_panel").resizable(true).min_height(60.0).max_height(300.0).show(ctx, |ui| {
            ui.add_space(5.0);
            ui.label("Log Output");
            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                let mut log_clone = self.adb_output.clone();
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut log_clone).font(egui::TextStyle::Monospace).interactive(false));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.installed_packages.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("Connect device and press 'List Packages'").weak());
                });
            } else if filtered_packages.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("No packages match your search or filters.").weak());
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(5.0);
                    for info in filtered_packages {
                        let is_selected = self.selected.contains(&info.id);
                        let frame_color = if is_selected { ui.style().visuals.widgets.hovered.bg_fill } else { ui.style().visuals.widgets.noninteractive.bg_fill };
                        let frame = egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                            .rounding(ui.style().visuals.widgets.noninteractive.rounding)
                            .fill(frame_color);
                        
                        let response = frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let mut selected_for_toggle = is_selected;
                                if ui.checkbox(&mut selected_for_toggle, "").clicked() {
                                    if selected_for_toggle { self.selected.insert(info.id.clone()); } else { self.selected.remove(&info.id); }
                                }
                                ui.label(egui::RichText::new(&info.id).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let status = info.removal.as_deref().unwrap_or("Unknown");
                                    let color = match status {
                                        "Recommended" => egui::Color32::from_rgb(0, 180, 255), // Light Blue
                                        "Advanced" => egui::Color32::from_rgb(255, 180, 0),    // Yellow
                                        "Expert" => egui::Color32::from_rgb(255, 80, 80),      // Red
                                        _ => ui.style().visuals.widgets.inactive.fg_stroke.color,
                                    };
                                    ui.label(egui::RichText::new(status).color(color).strong());
                                });
                            });
                            
                            if let Some(desc) = &info.description {
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(desc.replace("\\n", "\n")).text_style(egui::TextStyle::Small).weak());
                            }
                        });
                        if response.response.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        ui.add_space(6.0);
                    }
                });
            }
        });
    }
}
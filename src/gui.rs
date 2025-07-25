// src/gui.rs
use crate::{
    adb,
    config::{load_uad_list, PackageInfo},
};
use eframe::egui;
use std::collections::{BTreeSet, HashSet};
use std::sync::mpsc;
use std::thread;

enum AdbCommand {
    DetectDevice,
    ListPackages,
    Uninstall(Vec<String>),
    Reboot,
}

enum AdbResult {
    DeviceDetectionResult(String),
    PackageListResult(Result<Vec<String>, String>),
    UninstallProgress(usize, usize), // current, total
    UninstallFinished,
    RebootFinished,
}

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
    command_tx: mpsc::Sender<AdbCommand>,
    result_rx: mpsc::Receiver<AdbResult>,
    is_busy: bool,
    logo_texture: egui::TextureHandle,
    progress: f32,
    uninstall_total: usize,
}

impl DebloaterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_theme(&cc.egui_ctx);

        let uad_packages = load_uad_list();

        let all_lists_set: BTreeSet<String> =
            uad_packages.iter().filter_map(|p| p.list.clone()).collect();
        let mut all_lists: Vec<String> = all_lists_set.into_iter().collect();
        all_lists.insert(0, "All".to_string());

        let all_removals_set: BTreeSet<String> = uad_packages
            .iter()
            .filter_map(|p| p.removal.clone())
            .collect();
        let mut all_removals: Vec<String> = all_removals_set.into_iter().collect();
        all_removals.insert(0, "All".to_string());

        let (command_tx, command_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let result_tx_clone = result_tx.clone();

        thread::spawn(move || {
            while let Ok(command) = command_rx.recv() {
                let result = match command {
                    AdbCommand::DetectDevice => {
                        AdbResult::DeviceDetectionResult(adb::detect_device())
                    }
                    AdbCommand::ListPackages => {
                        let packages = adb::list_packages();
                        if !packages.is_empty() && packages[0].contains("Failed") {
                            AdbResult::PackageListResult(Err(packages[0].clone()))
                        } else {
                            AdbResult::PackageListResult(Ok(packages))
                        }
                    }
                    AdbCommand::Uninstall(packages) => {
                        let total = packages.len();
                        for (i, pkg) in packages.iter().enumerate() {
                            adb::uninstall(pkg);
                            let _ =
                                result_tx_clone.send(AdbResult::UninstallProgress(i + 1, total));
                        }
                        AdbResult::UninstallFinished
                    }
                    AdbCommand::Reboot => {
                        adb::reboot_device();
                        AdbResult::RebootFinished
                    }
                };
                let _ = result_tx.send(result);
            }
        });

        let image = image::load_from_memory(include_bytes!("../assets/img/logo.png"))
            .expect("Failed to load logo");
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        let logo_texture = cc
            .egui_ctx
            .load_texture("logo", color_image, Default::default());

        Self {
            uad_packages,
            installed_packages: HashSet::new(),
            selected: HashSet::new(),
            adb_output: "Welcome to P.U.R.G.E.!\n1. Connect your device with USB Debugging enabled.\n2. Click 'Detect Device'.\n3. Click 'List Packages' to begin.".to_string(),
            search_query: String::new(),
            all_lists,
            all_removals,
            filter_list: "All".to_string(),
            filter_removal: "All".to_string(),
            command_tx,
            result_rx,
            is_busy: false,
            logo_texture,
            progress: 0.0,
            uninstall_total: 0,
        }
    }

    fn handle_adb_results(&mut self) {
        if let Ok(result) = self.result_rx.try_recv() {
            match result {
                AdbResult::DeviceDetectionResult(output) => {
                    self.adb_output = output;
                    self.is_busy = false;
                }
                AdbResult::PackageListResult(Ok(installed)) => {
                    self.installed_packages = installed.into_iter().collect();
                    self.adb_output = format!(
                        "Found {} installed packages. List refreshed.",
                        self.installed_packages.len()
                    );
                    self.is_busy = false;
                }
                AdbResult::PackageListResult(Err(e)) => {
                    self.adb_output = e;
                    self.is_busy = false;
                }
                AdbResult::UninstallProgress(current, total) => {
                    self.progress = current as f32 / total as f32;
                    self.adb_output = format!("Purging package {} of {}...", current, total);
                }
                AdbResult::UninstallFinished => {
                    self.adb_output
                        .push_str("\nPurge complete. Refreshing package list...");
                    self.command_tx.send(AdbCommand::ListPackages).unwrap();
                    self.selected.clear();
                    self.progress = 0.0;
                    self.uninstall_total = 0;
                }
                AdbResult::RebootFinished => {
                    self.adb_output = "Reboot command sent to device.".to_string();
                    self.is_busy = false;
                }
            }
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
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);

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
        (
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.interact_size.y = 36.0;
    style.visuals.window_rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);

    ctx.set_style(style);
}

impl eframe::App for DebloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_adb_results();

        let filtered_packages: Vec<&PackageInfo> = self
            .uad_packages
            .iter()
            .filter(|info| {
                let is_installed = self.installed_packages.contains(&info.id);
                let matches_search = self.search_query.is_empty()
                    || info
                        .id
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase());
                let list_filter_pass =
                    self.filter_list == "All" || info.list.as_deref() == Some(&self.filter_list);
                let removal_filter_pass = self.filter_removal == "All"
                    || info.removal.as_deref() == Some(&self.filter_removal);
                is_installed && matches_search && list_filter_pass && removal_filter_pass
            })
            .collect();

        egui::SidePanel::left("control_panel")
            .width_range(300.0..=500.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        ui.image((self.logo_texture.id(), egui::vec2(64.0, 64.0)));
                        ui.heading("P.U.R.G.E.");
                    });
                    ui.label("Package Uninstaller & Resource Garbage Eliminator");
                    ui.add_space(20.0);

                    let group_frame = egui::Frame {
                        inner_margin: egui::Margin::same(12.0),
                        ..egui::Frame::group(ui.style())
                    };

                    group_frame.show(ui, |ui| {
                        ui.label(egui::RichText::new("1. Device Connection").strong());
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_enabled(!self.is_busy, egui::Button::new("🔌 Detect Device"))
                                .clicked()
                            {
                                self.is_busy = true;
                                self.adb_output = "Detecting device...".to_string();
                                self.command_tx.send(AdbCommand::DetectDevice).unwrap();
                            }
                            if ui
                                .add_enabled(!self.is_busy, egui::Button::new("📦 List Packages"))
                                .clicked()
                            {
                                self.is_busy = true;
                                self.adb_output = "Listing installed packages...".to_string();
                                self.command_tx.send(AdbCommand::ListPackages).unwrap();
                            }
                        });
                    });

                    ui.add_space(15.0);

                    group_frame.show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            let purge_button_text =
                                format!("🔥 Purge ({}) Selected", self.selected.len());
                            let purge_button = egui::Button::new(
                                egui::RichText::new(purge_button_text).size(20.0).strong(),
                            )
                            .rounding(egui::Rounding::same(18.0));

                            ui.add_enabled_ui(!self.selected.is_empty() && !self.is_busy, |ui| {
                                let response =
                                    ui.add_sized([ui.available_width(), 40.0], purge_button);
                                if response.clicked() {
                                    self.is_busy = true;
                                    self.uninstall_total = self.selected.len();
                                    self.adb_output =
                                        format!("Purging {} packages...", self.selected.len());
                                    let packages_to_uninstall: Vec<String> =
                                        self.selected.iter().cloned().collect();
                                    self.command_tx
                                        .send(AdbCommand::Uninstall(packages_to_uninstall))
                                        .unwrap();
                                }
                            });

                            if self.is_busy && self.uninstall_total > 0 {
                                ui.add_space(5.0);
                                ui.add(egui::ProgressBar::new(self.progress).show_percentage());
                            }

                            ui.add_space(10.0);
                            ui.separator();
                            ui.label("Other Actions");

                            let reboot_button = egui::Button::new("🔄 Reboot Device");
                            if ui.add_enabled(!self.is_busy, reboot_button).clicked() {
                                self.is_busy = true;
                                self.adb_output = "Sending reboot command...".to_string();
                                self.command_tx.send(AdbCommand::Reboot).unwrap();
                            }
                        });
                    });
                });
            });

        egui::TopBottomPanel::bottom("bottom_log_panel")
            .resizable(true)
            .default_height(150.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.visuals_mut().panel_fill = ui.style().visuals.widgets.noninteractive.bg_fill;
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("Log Output").strong());
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            let mut log_clone = self.adb_output.clone();
                            if self.is_busy && self.uninstall_total == 0 {
                                log_clone.push_str("\nWorking...");
                            }
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(&mut log_clone)
                                    .font(egui::TextStyle::Monospace)
                                    .interactive(false),
                            );
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!self.is_busy);
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 8.0;
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("🔎 Search...")
                        .desired_width(180.0),
                );

                egui::ComboBox::from_id_source("list_filter")
                    .selected_text(self.filter_list.clone())
                    .show_ui(ui, |ui| {
                        for list_name in &self.all_lists {
                            ui.selectable_value(
                                &mut self.filter_list,
                                list_name.clone(),
                                list_name.clone(),
                            );
                        }
                    });

                egui::ComboBox::from_id_source("removal_filter")
                    .selected_text(self.filter_removal.clone())
                    .show_ui(ui, |ui| {
                        for removal_name in &self.all_removals {
                            ui.selectable_value(
                                &mut self.filter_removal,
                                removal_name.clone(),
                                removal_name.clone(),
                            );
                        }
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌ Deselect All").clicked() {
                        self.selected.clear();
                    }
                    if ui.button("✅ Select All").clicked() {
                        for pkg in &filtered_packages {
                            self.selected.insert(pkg.id.clone());
                        }
                    }
                });
            });
            ui.separator();

            if self.is_busy && self.installed_packages.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                });
            } else if self.installed_packages.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("Connect device and press 'List Packages'").weak(),
                    );
                });
            } else if filtered_packages.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("No packages match your search or filters.").weak(),
                    );
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_enabled(!self.is_busy);
                    for info in filtered_packages {
                        let is_selected = self.selected.contains(&info.id);
                        let frame_color = if is_selected {
                            ui.style().visuals.selection.bg_fill
                        } else {
                            ui.style().visuals.widgets.noninteractive.bg_fill
                        };

                        let response = egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                            .rounding(ui.style().visuals.widgets.noninteractive.rounding)
                            .fill(frame_color)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let mut selected_for_toggle = is_selected;
                                    if ui.checkbox(&mut selected_for_toggle, "").clicked() {
                                        if selected_for_toggle {
                                            self.selected.insert(info.id.clone());
                                        } else {
                                            self.selected.remove(&info.id);
                                        }
                                    }
                                    ui.label(egui::RichText::new(&info.id).strong());
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            let status =
                                                info.removal.as_deref().unwrap_or("Unknown");
                                            let color = match status {
                                                "Recommended" => {
                                                    egui::Color32::from_rgb(0, 180, 255)
                                                }
                                                "Advanced" => egui::Color32::from_rgb(255, 180, 0),
                                                "Expert" => egui::Color32::from_rgb(255, 80, 80),
                                                _ => {
                                                    ui.style()
                                                        .visuals
                                                        .widgets
                                                        .inactive
                                                        .fg_stroke
                                                        .color
                                                }
                                            };
                                            ui.label(
                                                egui::RichText::new(status).color(color).strong(),
                                            );
                                        },
                                    );
                                });

                                if let Some(desc) = &info.description {
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(desc.replace("\\n", "\n"))
                                            .text_style(egui::TextStyle::Small)
                                            .weak(),
                                    );
                                }

                                if let Some(labels) = &info.labels {
                                    if !labels.is_empty() {
                                        ui.add_space(4.0);
                                        ui.horizontal_wrapped(|ui| {
                                            ui.style_mut().spacing.item_spacing.x = 4.0;
                                            ui.label(
                                                egui::RichText::new("Labels:")
                                                    .text_style(egui::TextStyle::Small)
                                                    .weak(),
                                            );
                                            for label in labels {
                                                egui::Frame::none()
                                                    .inner_margin(egui::vec2(4.0, 2.0))
                                                    .rounding(egui::Rounding::same(4.0))
                                                    .fill(
                                                        ui.style()
                                                            .visuals
                                                            .widgets
                                                            .noninteractive
                                                            .bg_fill,
                                                    )
                                                    .show(ui, |ui| {
                                                        ui.label(
                                                            egui::RichText::new(label)
                                                                .text_style(egui::TextStyle::Small),
                                                        );
                                                    });
                                            }
                                        });
                                    }
                                }

                                // ** THE FIX IS HERE **
                                if let Some(deps) = &info.dependencies {
                                    if !deps.is_empty() {
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "Dependencies: {}",
                                                deps.join(", ")
                                            ))
                                            .text_style(egui::TextStyle::Small)
                                            .weak(),
                                        );
                                    }
                                }

                                if let Some(needed) = &info.needed_by {
                                    if !needed.is_empty() {
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "Needed by: {}",
                                                needed.join(", ")
                                            ))
                                            .text_style(egui::TextStyle::Small)
                                            .weak(),
                                        );
                                    }
                                }
                            });

                        if response.response.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        ui.separator();
                    }
                });
            }
        });

        ctx.request_repaint();
    }
}

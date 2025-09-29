// src/gui/mod.rs
#![allow(deprecated)] // Allow RetainedImage for the entire module

mod theme;
mod widgets;

use crate::{
    adb,
    config::{load_uad_list, PackageInfo},
};
use eframe::egui;
use egui_extras::RetainedImage;
use std::collections::{BTreeSet, HashSet};
use std::sync::mpsc;
use std::thread;
use theme::{apply_theme, Theme};

#[derive(PartialEq)]
enum AppStatus {
    Ready,
    Busy,
    Error,
}
enum AdbCommand {
    Refresh,
    Uninstall(Vec<String>),
    Reboot,
}
enum AdbResult {
    RefreshSuccess(String, Vec<String>),
    RefreshFailure(String),
    UninstallProgress(usize, usize),
    UninstallFinished,
    RebootFinished,
}

struct TitleBarIcons {
    close: RetainedImage,
    minimize: RetainedImage,
    maximize: RetainedImage,
    restore: RetainedImage,
}

impl TitleBarIcons {
    fn new() -> Self {
        Self {
            close: RetainedImage::from_svg_bytes(
                "close.svg",
                include_bytes!("../../assets/icons/close.svg"),
            )
            .unwrap(),
            minimize: RetainedImage::from_svg_bytes(
                "minimize.svg",
                include_bytes!("../../assets/icons/window-minimize.svg"),
            )
            .unwrap(),
            maximize: RetainedImage::from_svg_bytes(
                "maximize.svg",
                include_bytes!("../../assets/icons/maximize.svg"),
            )
            .unwrap(),
            restore: RetainedImage::from_svg_bytes(
                "restore.svg",
                include_bytes!("../../assets/icons/minimize.svg"),
            )
            .unwrap(),
        }
    }
}

pub struct DebloaterApp {
    theme: Theme,
    uad_packages: Vec<PackageInfo>,
    installed_packages: HashSet<String>,
    selected: HashSet<String>,
    active_selection: Option<PackageInfo>,
    status_message: String,
    device_name: String,
    search_query: String,
    all_lists: Vec<String>,
    filter_list: String,
    all_removals: Vec<String>,
    filter_removal: String,
    command_tx: mpsc::Sender<AdbCommand>,
    result_rx: mpsc::Receiver<AdbResult>,
    logo_texture: egui::TextureHandle,
    progress: f32,
    app_status: AppStatus,
    title_bar_icons: TitleBarIcons,
}

impl DebloaterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::default();
        apply_theme(&cc.egui_ctx, &theme);
        let uad_packages = load_uad_list();

        let all_lists: Vec<String> = ["All".to_string()]
            .into_iter()
            .chain(
                uad_packages
                    .iter()
                    .filter_map(|p| p.list.clone())
                    .collect::<BTreeSet<String>>(),
            )
            .collect();
        let all_removals: Vec<String> = ["All".to_string()]
            .into_iter()
            .chain(
                uad_packages
                    .iter()
                    .filter_map(|p| p.removal.clone())
                    .collect::<BTreeSet<String>>(),
            )
            .collect();

        let (command_tx, command_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();

        thread::spawn(move || {
            while let Ok(command) = command_rx.recv() {
                let result = match command {
                    AdbCommand::Refresh => match adb::detect_device() {
                        Ok(_) => match adb::get_device_model() {
                            Ok(model) => match adb::list_packages() {
                                Ok(packages) => AdbResult::RefreshSuccess(model, packages),
                                Err(e) => AdbResult::RefreshFailure(e),
                            },
                            Err(e) => AdbResult::RefreshFailure(e),
                        },
                        Err(e) => AdbResult::RefreshFailure(e),
                    },
                    AdbCommand::Uninstall(packages) => {
                        let total = packages.len();
                        let tx = result_tx.clone();
                        for (i, pkg) in packages.iter().enumerate() {
                            adb::uninstall(pkg);
                            if tx.send(AdbResult::UninstallProgress(i + 1, total)).is_err() {
                                break;
                            }
                        }
                        AdbResult::UninstallFinished
                    }
                    AdbCommand::Reboot => {
                        adb::reboot_device();
                        AdbResult::RebootFinished
                    }
                };
                if result_tx.send(result).is_err() {
                    break;
                }
            }
        });

        let image = image::load_from_memory(include_bytes!("../../assets/img/logo.png")).unwrap();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [image.width() as _, image.height() as _],
            image.to_rgba8().as_flat_samples().as_slice(),
        );
        let logo_texture = cc
            .egui_ctx
            .load_texture("logo", color_image, Default::default());

        Self {
            theme,
            uad_packages,
            all_lists,
            all_removals,
            command_tx,
            result_rx,
            logo_texture,
            installed_packages: HashSet::new(),
            selected: HashSet::new(),
            active_selection: None,
            status_message: "Welcome! Connect your device to begin.".to_string(),
            device_name: "No Device Connected".to_string(),
            search_query: String::new(),
            filter_list: "All".to_string(),
            filter_removal: "All".to_string(),
            progress: 0.0,
            app_status: AppStatus::Ready,
            title_bar_icons: TitleBarIcons::new(),
        }
    }

    fn handle_adb_results(&mut self) {
        if let Ok(result) = self.result_rx.try_recv() {
            match result {
                AdbResult::RefreshSuccess(device, packages) => {
                    self.device_name = device;
                    self.installed_packages = packages.into_iter().collect();
                    self.status_message = "Ready.".to_string();
                    self.app_status = AppStatus::Ready;
                }
                AdbResult::RefreshFailure(e) => {
                    self.device_name = "No Device Connected".to_string();
                    self.installed_packages.clear();
                    self.status_message = format!("Error: {}", e);
                    self.app_status = AppStatus::Error;
                }
                AdbResult::UninstallProgress(current, total) => {
                    self.progress = current as f32 / total as f32;
                    self.status_message = format!("Purging {} of {}...", current, total);
                }
                AdbResult::UninstallFinished => {
                    self.status_message = "Purge complete. Refreshing...".to_string();
                    let _ = self.command_tx.send(AdbCommand::Refresh);
                    self.selected.clear();
                    self.active_selection = None;
                    self.progress = 0.0;
                }
                AdbResult::RebootFinished => {
                    self.status_message = "Reboot command sent.".to_string();
                    self.app_status = AppStatus::Ready;
                }
            }
        }
    }

    fn is_busy(&self) -> bool {
        self.app_status == AppStatus::Busy
    }

    fn draw_custom_title_bar(&mut self, ctx: &egui::Context) {
        let title_bar_height = 30.0;

        egui::TopBottomPanel::top("title_bar")
            .exact_height(title_bar_height)
            .frame(egui::Frame::none().fill(self.theme.title_bar))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let response = ui.interact(rect, ui.id(), egui::Sense::drag());
                if response.is_pointer_button_down_on() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.image((self.logo_texture.id(), egui::vec2(18.0, 18.0)));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("P.U.R.G.E.").strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let icon_size = egui::vec2(14.0, 14.0);

                        let close_button = egui::ImageButton::new((
                            self.title_bar_icons.close.texture_id(ctx),
                            icon_size,
                        ));
                        if ui.add(close_button).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                        let max_icon = if is_maximized {
                            &self.title_bar_icons.restore
                        } else {
                            &self.title_bar_icons.maximize
                        };
                        let maximize_button =
                            egui::ImageButton::new((max_icon.texture_id(ctx), icon_size));
                        if ui.add(maximize_button).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                        }

                        let minimize_button = egui::ImageButton::new((
                            self.title_bar_icons.minimize.texture_id(ctx),
                            icon_size,
                        ));
                        if ui.add(minimize_button).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });
    }

    fn draw_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("control_panel")
            .width_range(280.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(15.0);
                    ui.image((self.logo_texture.id(), egui::vec2(80.0, 80.0)));
                    ui.heading("P.U.R.G.E.");
                    ui.label(
                        egui::RichText::new("Package Uninstaller & Resource Garbage Eliminator")
                            .color(self.theme.on_surface_variant),
                    );
                });

                ui.add_space(25.0);
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(&self.device_name).size(22.0).strong());
                        let subtext = if self.device_name == "No Device Connected" {
                            ""
                        } else {
                            "Connected Device"
                        };
                        ui.label(egui::RichText::new(subtext).color(self.theme.on_surface_variant));
                    });
                    ui.add_space(10.0);
                    if ui
                        .add_sized(
                            [ui.available_width(), 40.0],
                            egui::Button::new("🔄 Refresh Connection"),
                        )
                        .clicked()
                    {
                        self.app_status = AppStatus::Busy;
                        self.status_message = "Scanning for devices...".to_string();
                        self.command_tx.send(AdbCommand::Refresh).unwrap();
                    }
                });
                ui.add_space(10.0);
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    let text = format!("🔥 Purge ({})", self.selected.len());
                    let button = egui::Button::new(egui::RichText::new(text).size(20.0).strong());
                    ui.style_mut().visuals.widgets.active.bg_fill = self.theme.danger;
                    ui.add_enabled_ui(!self.selected.is_empty() && !self.is_busy(), |ui| {
                        if ui.add_sized([ui.available_width(), 50.0], button).clicked() {
                            self.app_status = AppStatus::Busy;
                            let packages: Vec<String> = self.selected.iter().cloned().collect();
                            self.command_tx
                                .send(AdbCommand::Uninstall(packages))
                                .unwrap();
                        }
                    });
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(10.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        if ui
                            .add_sized(
                                [ui.available_width(), 35.0],
                                egui::Button::new("Reboot Device"),
                            )
                            .clicked()
                        {
                            self.app_status = AppStatus::Busy;
                            self.status_message = "Sending reboot command...".to_string();
                            self.command_tx.send(AdbCommand::Reboot).unwrap();
                        }
                    });
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        let (dot_color, spinner) = match self.app_status {
                            AppStatus::Ready => (self.theme.status_ok, false),
                            AppStatus::Busy => (self.theme.status_warn, true),
                            AppStatus::Error => (self.theme.status_err, false),
                        };
                        if spinner {
                            ui.spinner();
                        } else {
                            let (rect, _) = ui
                                .allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                            ui.painter().circle_filled(rect.center(), 5.0, dot_color);
                        }
                        ui.label(&self.status_message);
                    });
                });
            });
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context, filtered: Vec<PackageInfo>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_top_bar(ui, &filtered);
            ui.separator();
            if self.active_selection.is_some() {
                egui::SidePanel::right("detail_panel")
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: 15.0,
                        ..Default::default()
                    }))
                    .width_range(300.0..=ui.available_width() * 0.6)
                    .default_width(ui.available_width() * 0.4)
                    .show_inside(ui, |ui| {
                        self.draw_detail_panel(ui);
                    });
            }
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.draw_package_list(ui, filtered);
                });
        });
    }

    fn draw_top_bar(&mut self, ui: &mut egui::Ui, filtered: &[PackageInfo]) {
        ui.horizontal(|ui| {
            ui.add_space(5.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("🔎 Search...")
                        .desired_width(180.0)
                        .frame(false),
                );
            });
            egui::ComboBox::from_id_source("list_filter")
                .selected_text(&self.filter_list)
                .show_ui(ui, |ui| {
                    for name in &self.all_lists {
                        ui.selectable_value(&mut self.filter_list, name.clone(), name);
                    }
                });
            egui::ComboBox::from_id_source("removal_filter")
                .selected_text(&self.filter_removal)
                .show_ui(ui, |ui| {
                    for name in &self.all_removals {
                        ui.selectable_value(&mut self.filter_removal, name.clone(), name);
                    }
                });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Deselect All").clicked() {
                    self.selected.clear();
                }
                if ui.button("Select All").clicked() {
                    for pkg in filtered {
                        self.selected.insert(pkg.id.clone());
                    }
                }
            });
        });
    }

    fn draw_package_list(&mut self, ui: &mut egui::Ui, filtered: Vec<PackageInfo>) {
        if self.is_busy() && self.installed_packages.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.spinner();
            });
        } else if filtered.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("No packages match your filters.").weak());
            });
        } else {
            ui.set_enabled(!self.is_busy());
            for info in &filtered {
                let is_selected = self.selected.contains(&info.id);
                let is_active = self
                    .active_selection
                    .as_ref()
                    .map_or(false, |s| s.id == info.id);

                let response = ui
                    .scope(|ui| {
                        widgets::package_card(ui, &self.theme, &info, is_selected, is_active);
                    })
                    .response
                    .interact(egui::Sense::click());

                if response.clicked() {
                    if is_selected {
                        self.selected.remove(&info.id);
                    } else {
                        self.selected.insert(info.id.clone());
                    }
                    self.active_selection = Some(info.clone());
                }
                ui.add_space(6.0);
            }
        }
    }

    fn draw_detail_panel(&mut self, ui: &mut egui::Ui) {
        if let Some(info) = &self.active_selection {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(egui::RichText::new(&info.id).heading().strong());
                ui.separator();
                ui.add_space(10.0);
                if let Some(d) = &info.description {
                    ui.label(egui::RichText::new("Description").strong());
                    ui.label(d.replace("\\n", "\n"));
                    ui.add_space(10.0);
                }
                if let Some(l) = &info.labels {
                    if !l.is_empty() {
                        ui.label(egui::RichText::new("Labels").strong());
                        ui.horizontal_wrapped(|ui| {
                            for label in l {
                                ui.label(format!("#{}", label));
                            }
                        });
                        ui.add_space(10.0);
                    }
                }
                if let Some(d) = &info.dependencies {
                    if !d.is_empty() {
                        ui.label(egui::RichText::new("Dependencies").strong());
                        ui.label(d.join("\n"));
                        ui.add_space(10.0);
                    }
                }
                if let Some(n) = &info.needed_by {
                    if !n.is_empty() {
                        ui.label(egui::RichText::new("Needed By").strong());
                        ui.label(n.join("\n"));
                        ui.add_space(10.0);
                    }
                }
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Select a package for details").weak());
            });
        }
    }
}

impl eframe::App for DebloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_adb_results();
        let filtered: Vec<PackageInfo> = self
            .uad_packages
            .iter()
            .filter(|info| {
                self.installed_packages.contains(&info.id)
                    && (self.search_query.is_empty()
                        || info
                            .id
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase()))
                    && (self.filter_list == "All"
                        || info.list.as_deref() == Some(&self.filter_list))
                    && (self.filter_removal == "All"
                        || info.removal.as_deref() == Some(&self.filter_removal))
            })
            .cloned()
            .collect();

        self.draw_custom_title_bar(ctx);
        self.draw_side_panel(ctx);
        self.draw_central_panel(ctx, filtered);
        ctx.request_repaint();
    }
}

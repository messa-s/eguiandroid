use eframe::egui;
use std::path::PathBuf;

const ANDROID_ROOT: &str = "/storage/emulated/0";

pub struct App {
    picker_open: bool,
    dir: PathBuf,
    entries: Vec<(String, bool)>, // (name, is_dir)
    error: Option<String>,
    selected: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        // On desktop, start in $HOME so you can test without a phone
        let start = if cfg!(target_os = "android") {
            PathBuf::from(ANDROID_ROOT)
        } else {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/"))
        };
        Self {
            picker_open: false,
            dir: start,
            entries: Vec::new(),
            error: None,
            selected: None,
        }
    }
}

impl App {
    fn load_dir(&mut self, dir: PathBuf) {
        match std::fs::read_dir(&dir) {
            Ok(rd) => {
                let mut entries: Vec<(String, bool)> = rd
                    .flatten()
                    .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path().is_dir()))
                    .collect();
                // folders first, then alphabetical
                entries.sort_by(|a, b| {
                    b.1.cmp(&a.1)
                        .then(a.0.to_lowercase().cmp(&b.0.to_lowercase()))
                });
                self.entries = entries;
                self.dir = dir;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{}: {e}", dir.display())),
        }
    }

    fn picker_window(&mut self, ctx: &egui::Context) {
        let mut open = true;
        let mut goto: Option<PathBuf> = None;
        let mut chosen: Option<PathBuf> = None;

        egui::Window::new("Choose file")
            .open(&mut open)
            .collapsible(false)
            .default_width(300.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(self.dir.display().to_string());
                if ui.button("⬆ Up").clicked() {
                    if let Some(parent) = self.dir.parent() {
                        goto = Some(parent.to_path_buf());
                    }
                }
                if let Some(err) = &self.error {
                    ui.colored_label(egui::Color32::RED, err);
                }
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (name, is_dir) in &self.entries {
                        let label = if *is_dir {
                            format!("📁 {name}")
                        } else {
                            name.clone()
                        };
                        if ui.selectable_label(false, label).clicked() {
                            let path = self.dir.join(name);
                            if *is_dir {
                                goto = Some(path);
                            } else {
                                chosen = Some(path);
                            }
                        }
                    }
                });
            });

        if let Some(dir) = goto {
            self.load_dir(dir);
        }
        if let Some(path) = chosen {
            self.selected = Some(path);
            open = false;
        }
        self.picker_open = open;
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Choose file").clicked() {
                self.picker_open = true;
                self.load_dir(self.dir.clone());
            }
            match &self.selected {
                Some(p) => {
                    let name = p.file_name().unwrap_or_default().to_string_lossy();
                    ui.label(format!("Selected: {name}"));
                }
                None => {
                    ui.label("No file selected");
                }
            }
        });

        if self.picker_open {
            let ctx = ui.ctx().clone();
            self.picker_window(&ctx);
        }
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native("Jrrs", options, Box::new(|_cc| Ok(Box::<App>::default()))).unwrap();
}

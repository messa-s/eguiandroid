use eframe::egui;
use eguiandroid::App;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("Jrrs", options, Box::new(|_cc| Ok(Box::<App>::default())))
}
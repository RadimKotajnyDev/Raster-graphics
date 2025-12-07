mod vram;
mod app;
mod exercises;
mod tasks;
mod kernel;
mod utils;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        vsync: false,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Raster Graphics",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

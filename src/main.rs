mod vram;
mod app;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Raster Graphics",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

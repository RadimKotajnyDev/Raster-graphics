use eframe::egui;
use egui::{ColorImage, TextureHandle, Color32, Vec2};
use image::{ImageBuffer, Rgba};
use rfd::FileDialog;
use std::path::PathBuf;

/// Virtual framebuffer (like your V_RAM class)
struct VRam {
    width: u32,
    height: u32,
    data: Vec<u32>, // store ARGB pixels
}

impl VRam {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height) as usize],
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let argb = (255u32 << 24)
                | ((r as u32) << 16)
                | ((g as u32) << 8)
                | (b as u32);
            self.data[(y * self.width + x) as usize] = argb;
        }
    }

    fn to_color_image(&self) -> ColorImage {
        let mut pixels = Vec::with_capacity((self.width * self.height) as usize);
        for argb in &self.data {
            let a = ((*argb >> 24) & 0xFF) as u8;
            let r = ((*argb >> 16) & 0xFF) as u8;
            let g = ((*argb >> 8) & 0xFF) as u8;
            let b = (*argb & 0xFF) as u8;
            pixels.push(Color32::from_rgba_unmultiplied(r, g, b, a));
        }
        ColorImage {
            size: [self.width as usize, self.height as usize],
            source_size: Vec2::new(self.width as f32, self.height as f32),
            pixels,
        }
    }

    fn save_png(&self, path: &PathBuf) {
        let mut img = ImageBuffer::<Rgba<u8>, _>::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let argb = self.data[(y * self.width + x) as usize];
                let a = ((argb >> 24) & 0xFF) as u8;
                let r = ((argb >> 16) & 0xFF) as u8;
                let g = ((argb >> 8) & 0xFF) as u8;
                let b = (argb & 0xFF) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
        img.save(path).unwrap();
    }
}

/// Main application (like your MainWindow)
struct MyApp {
    vram: VRam,
    texture: Option<TextureHandle>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut vram = VRam::new(10, 10);

        vram.set_pixel(2, 2, 255, 0, 0);
        vram.set_pixel(7, 7, 0, 255, 0);
        vram.set_pixel(2, 7, 0, 0, 255);
        vram.set_pixel(7, 2, 255, 255, 255);

        let texture = Some(cc.egui_ctx.load_texture(
            "framebuffer",
            vram.to_color_image(),
            egui::TextureOptions::NEAREST,
        ));

        Self { vram, texture }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if ui.button("Load Image").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(img) = image::open(&path) {
                        let rgba = img.to_rgba8();
                        self.vram = VRam::new(rgba.width(), rgba.height());
                        for (x, y, pixel) in rgba.enumerate_pixels() {
                            let [r, g, b, _a] = pixel.0;
                            self.vram.set_pixel(x, y, r, g, b);
                        }
                        self.texture = Some(ctx.load_texture(
                            "framebuffer",
                            self.vram.to_color_image(),
                            egui::TextureOptions::NEAREST,
                        ));
                    }
                }
            }

            if ui.button("Save as PNG").clicked() {
                if let Some(path) = FileDialog::new().save_file() {
                    self.vram.save_png(&path);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tex) = &self.texture {
                // Get available space
                let available = ui.available_size();

                // Calculate scale to fit image to window while maintaining aspect ratio
                let img_aspect = self.vram.width as f32 / self.vram.height as f32;
                let win_aspect = available.x / available.y;

                let size = if img_aspect > win_aspect {
                    // Image is wider - fit to width
                    Vec2::new(available.x, available.x / img_aspect)
                } else {
                    // Image is taller - fit to height
                    Vec2::new(available.y * img_aspect, available.y)
                };

                // Center the image
                ui.centered_and_justified(|ui| {
                    ui.image((tex.id(), size));
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("Raster Graphics", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))))
}
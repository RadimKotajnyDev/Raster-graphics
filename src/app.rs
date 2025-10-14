use crate::vram::VRam;
use eframe::egui::{self, TextureHandle, Vec2};
use crate::exercises;

pub struct MyApp {
    pub vram: VRam,
    pub texture: Option<TextureHandle>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let vram = VRam::new(256, 256);

        // exercises::cv01_rgb::exercise_one(&mut vram);

        // exercises::cv02_images::grayscale(&mut vram);
        // exercises::cv02_images::saturate_image(&mut vram, 0.5);

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
            ui.horizontal(|ui| {
                if ui.button("Load Image").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        if let Ok(img) = image::open(&path) {
                            // Load the imported image directly into VRAM
                            self.vram.set_from_dynamic_image(&img);

                            // Make changes to the uploaded image here:
                            exercises::cv02_images::hue_shift(&mut self.vram, -270);

                            // Update texture after processing
                            self.texture = Some(ctx.load_texture(
                                "framebuffer",
                                self.vram.to_color_image(),
                                egui::TextureOptions::NEAREST,
                            ));
                        }
                    }
                }

                if ui.button("Save as PNG").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.vram.save_png(&path);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tex) = &self.texture {
                let available = ui.available_size();

                let img_aspect = self.vram.width as f32 / self.vram.height as f32;
                let win_aspect = available.x / available.y;

                let size = if img_aspect > win_aspect {
                    Vec2::new(available.x, available.x / img_aspect)
                } else {
                    Vec2::new(available.y * img_aspect, available.y)
                };

                ui.centered_and_justified(|ui| {
                    ui.image((tex.id(), size));
                });
            }
        });
    }
}

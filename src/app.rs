use crate::vram::VRam;
use eframe::egui::{self, TextureHandle, Vec2};
use crate::exercises;
use std::time::{Duration, Instant};

pub struct MyApp {
    pub vram: VRam,
    pub texture: Option<TextureHandle>,
    // UI state
    saturation: f32,
    hue: f32,
    // Debounce support
    last_edit_change: Option<Instant>,
    debounce: Duration,
    // Keep original image to re-apply effects without compounding
    original_vram: VRam,
    // Persistent menu visibility
    show_edit_menu: bool,
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

        Self {
            original_vram: vram.clone(),
            vram,
            texture,
            saturation: 0.0,
            hue: 0.0,
            last_edit_change: None,
            debounce: Duration::from_millis(300),
            show_edit_menu: false,
        }
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
                            // Keep a pristine copy for re-applying effects
                            self.original_vram = self.vram.clone();

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
                    if let Some(mut path) = rfd::FileDialog::new().save_file() {
                        // Ensure the file has a .png extension to avoid crashes in the image encoder
                        let is_png = path
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|ext| ext.eq_ignore_ascii_case("png"))
                            .unwrap_or(false);
                        if !is_png {
                            path.set_extension("png");
                        }
                        self.vram.save_png(&path);
                    }
                }

                if ui.button("Edit image").clicked() {
                    self.show_edit_menu = true;
                }
            });
        });

        // Persistent Edit image menu window (closes only on outside click)
        if self.show_edit_menu {
            if let Some(win) = egui::Window::new("Edit image")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_edit_menu)
                .show(ctx, |ui| {
                    let saturate_interaction = ui.add(
                        egui::Slider::new(&mut self.saturation, -1.0..=1.0).text("Saturation")
                    );

                    let hue_interaction = ui.add(
                        egui::Slider::new(&mut self.hue, 0.0..=360.0).text("Hue (deg 0-360)")
                    );

                    if saturate_interaction.changed() || hue_interaction.changed() {
                        self.last_edit_change = Some(Instant::now());
                    }

                    // Apply debounced update after a quiet period
                    let should_apply = self
                        .last_edit_change
                        .map(|t| t.elapsed() >= self.debounce)
                        .unwrap_or(false);

                    if should_apply {
                        // Reset to original and apply effect
                        self.vram = self.original_vram.clone();

                        // Apply all current adjustments; debounce is based on time since last change
                        exercises::cv02_images::saturate_image(&mut self.vram, self.saturation);
                        exercises::cv02_images::hue_shift(&mut self.vram, self.hue.round() as i32);

                        // Update texture after processing
                        self.texture = Some(ctx.load_texture(
                            "framebuffer",
                            self.vram.to_color_image(),
                            egui::TextureOptions::NEAREST,
                        ));

                        // Clear pending state
                        self.last_edit_change = None;
                    }
                })
            {
                let rect = win.response.rect;
                let clicked_outside = ctx.input(|i| {
                    i.pointer.any_pressed()
                        && i.pointer
                            .interact_pos()
                            .map_or(false, |p| !rect.contains(p))
                });
                if clicked_outside {
                    self.show_edit_menu = false;
                }
            }
        }

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
                    let resp = ui.image((tex.id(), size));
                    if self.last_edit_change.is_some() {
                        let rect = resp.rect;
                        let painter = ui.painter();
                        painter.rect_filled(rect, 0.0, egui::Color32::from_black_alpha(160));
                        let sp_size = Vec2::splat(24.0);
                        let sp_rect = egui::Rect::from_center_size(rect.center(), sp_size);
                        ui.put(sp_rect, egui::Spinner::new());
                    }
                });
            }
        });
    }
}

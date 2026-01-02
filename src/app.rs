use crate::exercises;
use crate::kernel::Kernel;
use crate::tasks;
use crate::vram::VRam;
use eframe::egui::{self, TextureHandle, Vec2};
use std::time::{Duration, Instant};
use crate::utils::point::Point;

pub struct MyApp {
    pub vram: VRam,
    pub texture: Option<TextureHandle>,
    saturation: f32,
    hue: f32,
    last_edit_change: Option<Instant>,
    debounce: Duration,
    original_vram: VRam,
    show_edit_menu: bool,
    pub is_change_pending: bool,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let vram = VRam::new(700, 500);

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
            is_change_pending: false,
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

                            self.vram.set_from_dynamic_image(&img);

                            self.original_vram = self.vram.clone();

                            self.texture = Some(ctx.load_texture(
                                "framebuffer",
                                self.vram.to_color_image(),
                                egui::TextureOptions::NEAREST,
                            ));
                        }
                    }
                }

                if ui.button("Save as PNG").clicked() {
                    if let Some(mut path) = rfd::FileDialog::new()
                        .add_filter("PNG Image", &["png"])
                        .save_file() {
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

        if self.show_edit_menu {
            if let Some(win) = egui::Window::new("Edit image")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .open(&mut self.show_edit_menu)
                .show(ctx, |ui| {
                    let saturate_interaction = ui.add(
                        egui::Slider::new(&mut self.saturation, -1.0..=1.0).text("Saturation"),
                    );

                    let hue_interaction = ui
                        .add(egui::Slider::new(&mut self.hue, 0.0..=360.0).text("Hue (deg 0-360)"));

                    let convolution_button = ui.add(egui::Button::new("Convolution"));

                    let red_eye_removal_button = ui.add(egui::Button::new("Remove red eyes"));

                    let convolution_smoothing_button = ui.add(egui::Button::new("Convolution smoothing"));

                    if saturate_interaction.changed() || hue_interaction.changed() {
                        self.last_edit_change = Some(Instant::now());
                        self.is_change_pending = false;
                    }

                    if red_eye_removal_button.clicked() {
                        let snapshot_start = Instant::now();

                        tasks::ku1::red_eye_removal(&mut self.vram);

                        let duration = snapshot_start.elapsed();
                        println!("Red eye removal took: {:.2?}", duration);

                        self.is_change_pending = true;
                        self.last_edit_change = Some(Instant::now());
                    }

                    if convolution_smoothing_button.clicked() {
                        let snapshot_start = Instant::now();

                        let kernel = Kernel::create_gaussian_blur();
                        tasks::ku1::convolution_smoothing(&mut self.vram, &kernel, 15);

                        let duration = snapshot_start.elapsed();
                        println!("Convolution smoothing took: {:.2?}", duration);

                        self.is_change_pending = true;
                        self.last_edit_change = Some(Instant::now());
                    }

                    if convolution_button.clicked() {
                        let snapshot_start = Instant::now();

                        exercises::cv03_convolution::convolution(&mut self.vram);

                        let duration = snapshot_start.elapsed();
                        println!("Convolution took: {:.2?}", duration);

                        self.is_change_pending = true;
                        self.last_edit_change = Some(Instant::now());
                    }

                    if ui.button("Draw KU2 Spline (Wave)").clicked() {
                        let snapshot_start = Instant::now();

                        let control_points = vec![
                            Point::new(50.0, 200.0),
                            Point::new(150.0, 50.0),
                            Point::new(250.0, 250.0),
                            Point::new(350.0, 100.0),
                            Point::new(450.0, 250.0),
                            Point::new(550.0, 150.0),
                        ];

                        self.vram = VRam::new(self.vram.width, self.vram.height);

                        tasks::ku2::draw_bezier_spline(&mut self.vram, &control_points, 0.01);

                        let duration = snapshot_start.elapsed();
                        println!("KU2 Spline drawing took: {:.2?}", duration);

                        self.is_change_pending = true;
                        self.last_edit_change = Some(Instant::now());
                    }
                    
                    if ui.button("KU3: Analog Clock (8:18:35)").clicked() {
                        let snapshot_start = Instant::now();

                        let target_time = tasks::ku3::ClockTime {
                            hours: 8,
                            minutes: 18,
                            seconds: 35,
                        };

                        tasks::ku3::draw_clock(&mut self.vram, target_time);

                        let duration = snapshot_start.elapsed();
                        println!("Clock composition took: {:.2?}", duration);

                        self.is_change_pending = true;
                        self.last_edit_change = Some(Instant::now());
                    }

                    let should_apply = self
                        .last_edit_change
                        .map(|t| t.elapsed() >= self.debounce)
                        .unwrap_or(false);

                    if should_apply {
                        if !self.is_change_pending {
                            let snapshot_start = Instant::now();

                            // Reset to original and apply all changes
                            self.vram = self.original_vram.clone();

                            if self.saturation != 0.0 {
                                exercises::cv02_images::saturate_image(&mut self.vram, self.saturation);
                            }

                            if self.hue != 0.0 {
                                exercises::cv02_images::hue_shift(&mut self.vram, self.hue.round() as i32);
                            }

                            let duration = snapshot_start.elapsed();
                            println!("Image processing took: {:.2?}", duration);
                        }

                        self.texture = Some(ctx.load_texture(
                            "framebuffer",
                            self.vram.to_color_image(),
                            egui::TextureOptions::NEAREST,
                        ));

                        self.last_edit_change = None;
                        self.is_change_pending = false;
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
                if self.last_edit_change.is_some() {
                    ctx.request_repaint_after(Duration::from_millis(16));
                }
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

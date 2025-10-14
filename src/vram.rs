use egui::{Color32, ColorImage, Vec2};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::path::PathBuf;

/// Virtual framebuffer (like your V_RAM class)
pub struct VRam {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>, // store ARGB pixels
}

impl VRam {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height) as usize],
        }
    }

    pub fn get_pixel_rgb(&self, x: u32, y: u32) -> Option<(u8, u8, u8)> {
        if x < self.width && y < self.height {
            let argb = self.data[(y * self.width + x) as usize];
            let r = ((argb >> 16) & 0xFF) as u8;
            let g = ((argb >> 8) & 0xFF) as u8;
            let b = (argb & 0xFF) as u8;
            Some((r, g, b))
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let argb = (255u32 << 24)
                | ((r as u32) << 16)
                | ((g as u32) << 8)
                | (b as u32);
            self.data[(y * self.width + x) as usize] = argb;
        }
    }

    /// Replace VRAM contents from an RGBA image (ignores alpha for now).
    pub fn set_from_rgba8(&mut self, rgba: &RgbaImage) {
        self.width = rgba.width();
        self.height = rgba.height();
        self.data = vec![0; (self.width * self.height) as usize];
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let [r, g, b, _a] = pixel.0;
            self.set_pixel(x, y, r, g, b);
        }
    }

    /// Replace VRAM contents from any DynamicImage.
    pub fn set_from_dynamic_image(&mut self, img: &DynamicImage) {
        let rgba = img.to_rgba8();
        self.set_from_rgba8(&rgba);
    }

    pub fn to_color_image(&self) -> ColorImage {
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

    pub fn save_png(&self, path: &PathBuf) {
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
        img.save(path).expect("Failed to save PNG");
    }
}

use crate::utils::converters::{rgb_to_hsl, hsl_to_rgb, HSL};
use crate::vram::VRam;
use crate::kernel::Kernel;

pub fn red_eye_removal(vram: &mut VRam) {
    for y in 0..vram.height {
        for x in 0..vram.width {
            if let Some((r, g, b)) = vram.get_pixel_rgb(x, y) {

                let hsl: HSL = rgb_to_hsl(r, g, b);

                let is_red = (hsl.hue <= 20.0) || (hsl.hue >= 340.0);
                let is_saturated = hsl.saturation > 0.35;
                let is_midlight = hsl.lightness > 0.05 && hsl.lightness < 0.7;

                if is_red && is_saturated && is_midlight {
                    let new_sat = hsl.saturation * 0.15;
                    let new_light = hsl.lightness * 0.6;

                    const TARGET_HUE: f32 = 30.0;
                    const LERP_FACTOR: f32 = 0.6;

                    let mut diff = TARGET_HUE - hsl.hue;

                    if diff > 180.0 {
                        diff -= 360.0;
                    } else if diff < -180.0 {
                        diff += 360.0;
                    }
                    let final_hue = (hsl.hue + diff * LERP_FACTOR + 360.0) % 360.0;

                    let new_rgb = hsl_to_rgb(final_hue, new_sat, new_light);
                    vram.set_pixel(x, y, new_rgb.r, new_rgb.g, new_rgb.b);
                }
            }
        }
    }
}

pub fn convolution_smoothing(vram: &mut VRam, kernel: &Kernel, threshold: i32) {
    let source = vram.clone();
    let mut blurred_result = vram.clone();

    let kw = kernel.width as i32;
    let kh = kernel.height as i32;
    let half_w = kw / 2;
    let half_h = kh / 2;
    let divider = if kernel.divider == 0 { 1 } else { kernel.divider };

    for y in 0..vram.height as i32 {
        for x in 0..vram.width as i32 {
            let mut sum_r = 0;
            let mut sum_g = 0;
            let mut sum_b = 0;

            for ky in 0..kh {
                for kx in 0..kw {
                    // Clamp to edge
                    let px = (x + kx - half_w).clamp(0, vram.width as i32 - 1);
                    let py = (y + ky - half_h).clamp(0, vram.height as i32 - 1);

                    if let Some((r, g, b)) = source.get_pixel_rgb(px as u32, py as u32) {
                        let w = kernel.get(kx as usize, ky as usize);
                        sum_r += w * r as i32;
                        sum_g += w * g as i32;
                        sum_b += w * b as i32;
                    }
                }
            }

            sum_r /= divider;
            sum_g /= divider;
            sum_b /= divider;

            let r = sum_r.clamp(0, 255) as u8;
            let g = sum_g.clamp(0, 255) as u8;
            let b = sum_b.clamp(0, 255) as u8;

            blurred_result.set_pixel(x as u32, y as u32, r, g, b);
            vram.set_pixel(x as u32, y as u32, r, g, b);
        }
    }

    for y in 0..vram.height as i32 {
        for x in 0..vram.width as i32 {
            let original = source.get_pixel_rgb(x as u32, y as u32).unwrap();
            let blurred = blurred_result.get_pixel_rgb(x as u32, y as u32).unwrap();

            let diff = (blurred.0 as i32 - original.0 as i32).abs();
            if diff < threshold {
                vram.set_pixel(x as u32, y as u32, blurred.0, blurred.1, blurred.2);
            } else {
                vram.set_pixel(x as u32, y as u32, original.0, original.1, original.2);
            }
        }
    }
}

use crate::utils::{rgb_to_hsl, hsl_to_rgb, HSL};
use crate::vram::VRam;

pub fn red_eye_removal(vram: &mut VRam) {
    for y in 1..vram.width {
        for x in 1..vram.width {
            if let Some((r, g, b)) = vram.get_pixel_rgb(x, y) {
                let mut hsl: HSL = rgb_to_hsl(r, g, b);


                // Detect typical "red" hue
                // hue in [0.0, 360.0]
                let is_red = (hsl.hue <= 20.0) || (hsl.hue >= 340.0);
                let is_saturated = hsl.saturation > 0.35;
                let is_midlight = hsl.lightness > 0.05 && hsl.lightness < 0.7;

                if is_red && is_saturated && is_midlight {
                    // Desaturate strongly and darken
                    hsl.saturation *= 0.15;
                    hsl.lightness *= 0.6;

                    // Optional: slight hue shift towards brown (30°)
                    let target_hue = 30.0;
                    let lerp_factor = 0.6;
                    let mut diff = target_hue - hsl.hue;
                    if diff > 180.0 {
                        diff -= 360.0;
                    } else if diff < -180.0 {
                        diff += 360.0;
                    }
                    hsl.hue = (hsl.hue + diff * lerp_factor + 360.0) % 360.0;

                    let new_rgb = hsl_to_rgb(hsl.hue, hsl.saturation, hsl.lightness);
                    vram.set_pixel(x, y, new_rgb.r, new_rgb.g, new_rgb.b);
                }
            }
        }
    }
}
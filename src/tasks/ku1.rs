use crate::utils::{rgb_to_hsl, hsl_to_rgb, HSL};
use crate::vram::VRam;

pub fn red_eye_removal(vram: &mut VRam) {
    for y in 0..vram.height {
        for x in 0..vram.width {
            if let Some((r, g, b)) = vram.get_pixel_rgb(x, y) {

                let hsl: HSL = rgb_to_hsl(r, g, b);

                let is_red = (hsl.hue <= 50.0) || (hsl.hue >= 300.0);
                let is_saturated = hsl.saturation > 0.35;
                let is_midlight = hsl.lightness > 0.05 && hsl.lightness < 0.7;

                if is_red && is_saturated && is_midlight {
                    // Desaturate strongly and darken
                    let new_sat = hsl.saturation * 0.15;
                    let new_light = hsl.lightness * 0.6;

                    // Optional: slight hue shift towards brown (30°)
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

/// Aplikuje konvoluční vyhlazení s prahem podle zadání.
/// F(x,y) = (a ⊗ f)(x,y),  pokud |(a ⊗ f)(x,y) - f(x,y)| < T
/// F(x,y) = f(x,y),        pokud |(a ⊗ f)(x,y) - f(x,y)| >= T
///
/// Kde (a ⊗ f)(x,y) je průměr 3x3 okolí a f(x,y) je původní pixel.
/// Porovnání se provádí na základě grayscale hodnot, ale aplikuje se plná RGB barva.
pub fn convolution_smoothing(vram: &mut VRam) {
    // Prahová hodnota T. Můžeš experimentovat s touto hodnotou.
    const T: f32 = 30.0;

    // Váhy pro převod na grayscale
    const R_WEIGHT: f32 = 0.299;
    const G_WEIGHT: f32 = 0.587;
    const B_WEIGHT: f32 = 0.114;

    // Vytvoříme kopii VRAM, ze které budeme číst původní hodnoty.
    // To je klíčové, abychom nečetli již upravené pixely z aktuální iterace.
    let original_vram = vram.clone();

    // Projdeme všechny vnitřní pixely (vynecháme 1px okraj pro zjednodušení)
    for y in 1..(original_vram.height - 1) {
        for x in 1..(original_vram.width - 1) {

            // 1. Získáme původní barvu centrálního pixelu
            if let Some((r, g, b)) = original_vram.get_pixel_rgb(x, y) {

                // 2. Spočítáme grayscale hodnotu původního pixelu f(x,y)
                let f_xy = R_WEIGHT * r as f32 + G_WEIGHT * g as f32 + B_WEIGHT * b as f32;

                // 3. Spočítáme průměr 3x3 okolí (a ⊗ f)(x,y)
                let mut sum_r: f32 = 0.0;
                let mut sum_g: f32 = 0.0;
                let mut sum_b: f32 = 0.0;

                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let nx = (x as i32 + kx) as u32;
                        let ny = (y as i32 + ky) as u32;

                        if let Some((nr, ng, nb)) = original_vram.get_pixel_rgb(nx, ny) {
                            sum_r += nr as f32;
                            sum_g += ng as f32;
                            sum_b += nb as f32;
                        }
                    }
                }

                let avg_r = sum_r / 9.0;
                let avg_g = sum_g / 9.0;
                let avg_b = sum_b / 9.0;

                // 4. Spočítáme grayscale hodnotu zprůměrovaného pixelu
                let a_f_xy = R_WEIGHT * avg_r + G_WEIGHT * avg_g + B_WEIGHT * avg_b;

                // 5. Aplikujeme formuli s prahem T
                let diff = (a_f_xy - f_xy).abs();

                if diff < T {
                    // Rozdíl je malý, pixel vyhladíme (použijeme průměrnou hodnotu)
                    vram.set_pixel(
                        x,
                        y,
                        avg_r.clamp(0.0, 255.0) as u8,
                        avg_g.clamp(0.0, 255.0) as u8,
                        avg_b.clamp(0.0, 255.0) as u8
                    );
                } else {
                    // Rozdíl je velký (pravděpodobně hrana), ponecháme původní pixel
                    vram.set_pixel(x, y, r, g, b);
                }

            }
        }
    }
    // Okrajové pixely (y=0, x=0, y=height-1, x=width-1) jsou záměrně
    // ponechány beze změny, protože jsme je v smyčce přeskočili.
}
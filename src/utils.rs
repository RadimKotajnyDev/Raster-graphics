pub struct Hsl {
    hue: f32,
    saturation: f32,
    lightness: f32
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> Hsl {
    // normalized values
    let norm_r = (r as f32) / 255.0;
    let norm_g = (g as f32) / 255.0;
    let norm_b = (b as f32) / 255.0;

    let max = norm_r.max(norm_g).max(norm_b);
    let min = norm_r.min(norm_g).min(norm_b);
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    let saturation = if delta == 0.0 {
        0.0 // if delta = 0, the pixel is gray, no saturation
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };


    /*
    the % operator on floats in Rust doesn’t wrap around like in math — it’s remainder, not modulus.
     */
    let mut hue = if delta == 0.0 {
        0.0
    } else if max == norm_r {
        (norm_g - norm_b) / delta % 6.0
    } else if max == norm_g {
        ((norm_b - norm_r) / delta) + 2.0
    } else {
        ((norm_r - norm_g) / delta) + 4.0
    };

    hue *= 60.0;
    if hue < 0.0 {
        hue += 360.0;
    }

    // return statement
    Hsl { hue, saturation, lightness }
}
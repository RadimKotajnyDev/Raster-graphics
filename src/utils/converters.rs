pub struct HSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> HSL {
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

    HSL { hue, saturation, lightness }
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> RGB {
    // Normalize hue to [0, 360)
    let mut hue = h % 360.0;
    if hue < 0.0 { hue += 360.0; }

    let saturation = if s < 0.0 { 0.0 } else if s > 1.0 { 1.0 } else { s };
    let lightness = if l < 0.0 { 0.0 } else if l > 1.0 { 1.0 } else { l };

    if saturation == 0.0 {
        let v = (lightness * 255.0).round().clamp(0.0, 255.0) as u8;
        return RGB { r: v, g: v, b: v };
    }

    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let m = lightness - c / 2.0;

    let (r1, g1, b1) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    RGB { r, g, b }
}
use crate::vram::VRam;

// 2 Práce s obrazem - Grayscale, Saturace, Hue
pub fn grayscale(vram: &mut VRam) {
    for y in 0..vram.height {
        for x in 0..vram.width {
            if let Some((r, g, b)) = vram.get_pixel_rgb(x, y) {
                // Compute luminance using weighted average (no division by 3)
                let l = (0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32);
                vram.set_pixel(x, y, l as u8, l as u8, l as u8);
            }
        }
    }
}

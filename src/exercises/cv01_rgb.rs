use crate::vram::VRam;

pub fn exercise_one(vram: &mut VRam) {
    const BLUE: u8 = 128;

    for y in 0..vram.height {
        for x in 0..vram.width {
            vram.set_pixel(x, y, x as u8, y as u8, BLUE);
        }
    }
}
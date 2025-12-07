use crate::utils::point::Point;
use crate::vram::VRam;

pub fn draw_line(vram: &mut VRam, p1: Point, p2: Point, r: u8, g: u8, b: u8) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    let steps = dx.abs().max(dy.abs());

    let x_inc = dx / steps;
    let y_inc = dy / steps;

    let mut x = p1.x;
    let mut y = p1.y;

    for _i in 1..=(steps.ceil() as u32) {
        x += x_inc;
        y += y_inc;

        if x >= 0.0 && x < vram.width as f32 && y >= 0.0 && y < vram.height as f32 {
            vram.set_pixel(x.round() as u32, y.round() as u32, r, g, b);
        }
    }
}
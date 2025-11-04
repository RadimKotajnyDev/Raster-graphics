#![allow(dead_code)]
use crate::vram::VRam;

pub fn convolution(vram: &mut VRam) {
    let kernel = vec![
        vec![1, 1, 1],
        vec![1, 1, 1],
        vec![1, 1, 1],
    ];

    for y in 1..vram.height as i32 - 1 {
        for x in 1..vram.width as i32 - 1 {
            let mut sum_r: i32 = 0;
            let mut sum_g: i32 = 0;
            let mut sum_b: i32 = 0;

            for ky in -1..=1 {
                for kx in -1..=1 {

                    //todo: fix this

                    if let Some((r, g, b)) = vram.get_pixel_rgb((x + kx) as u32, (y + ky) as u32) {
                        let weight = kernel[(ky + 1) as usize][(kx + 1) as usize];

                        sum_r += weight * r as i32;
                        sum_g += weight * g as i32;
                        sum_b += weight * b as i32;
                    }
                }
            }

            vram.set_pixel(
                x as u32,
                y as u32,
                (sum_r / 9) as u8,
                (sum_g / 9) as u8,
                (sum_b / 9) as u8,
            );
        }
    }
}

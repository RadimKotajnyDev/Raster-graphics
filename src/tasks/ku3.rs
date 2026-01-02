use crate::vram::VRam;
use image::{DynamicImage, GenericImageView, RgbaImage};
use std::path::Path;

pub fn draw_clock(vram: &mut VRam) {
    let base_path = Path::new("public/hodiny");

    let cifernik = load_image(&base_path.join("cifernikB.png"));
    let hodinovka = load_image(&base_path.join("hodinovka.png"));
    let minutovka = load_image(&base_path.join("minutovka.png"));
    let sekundovka = load_image(&base_path.join("sekundovka.png"));

    if let (Ok(cif), Ok(hod), Ok(min), Ok(sek)) = (cifernik, hodinovka, minutovka, sekundovka) {
        vram.set_from_dynamic_image(&cif);

        let center_x = vram.width / 2;
        let center_y = vram.height / 2;

        // Pozor: image crate používá radiány pro rotaci, musíme převést stupně.
        let rot_hod = rotate_image(&hod, 249.3); // 8:18:35
        let rot_min = rotate_image(&min, 111.5);
        let rot_sek = rotate_image(&sek, 210.0);

        blend_image_on_vram(vram, &rot_hod, center_x, center_y);
        blend_image_on_vram(vram, &rot_min, center_x, center_y);
        blend_image_on_vram(vram, &rot_sek, center_x, center_y);

        println!("KU3: Hodiny vykresleny pro čas 8:18:35");
    } else {
        eprintln!("Chyba: Nepodařilo se načíst obrázky ze složky 'hodiny/'.");
    }
}

fn load_image(path: &Path) -> Result<DynamicImage, image::ImageError> {
    image::open(path)
}

/// Manuální implementace rotace obrázku kolem střed    u
/// Využívá inverzní mapování a metodu nejbližšího souseda (Nearest Neighbor)
fn rotate_image(img: &DynamicImage, degrees: f32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let mut output = RgbaImage::new(w, h);

    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;

    let radians = degrees.to_radians();
    let cos_angle = radians.cos();
    let sin_angle = radians.sin();

    for y in 0..h {
        for x in 0..w {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            let src_x = dx * cos_angle + dy * sin_angle + cx;
            let src_y = -dx * sin_angle + dy * cos_angle + cy;

            let isrc_x = src_x.round() as i32;
            let isrc_y = src_y.round() as i32;

            if isrc_x >= 0 && isrc_x < w as i32 && isrc_y >= 0 && isrc_y < h as i32 {
                let pixel = img.get_pixel(isrc_x as u32, isrc_y as u32);
                output.put_pixel(x, y, pixel);
            }
        }
    }
    output
}

fn blend_image_on_vram(vram: &mut VRam, overlay: &RgbaImage, target_cx: u32, target_cy: u32) {
    let (w, h) = overlay.dimensions();

    let offset_x = (target_cx as i32) - (w as i32 / 2);
    let offset_y = (target_cy as i32) - (h as i32 / 2);

    for y in 0..h {
        for x in 0..w {
            let dest_x = offset_x + x as i32;
            let dest_y = offset_y + y as i32;

            if dest_x >= 0 && dest_x < vram.width as i32 && dest_y >= 0 && dest_y < vram.height as i32 {
                let fg_pixel = overlay.get_pixel(x, y);
                let alpha = fg_pixel[3] as f32 / 255.0;

                if alpha <= 0.001 {
                    continue;
                }

                if let Some((bg_r, bg_g, bg_b)) = vram.get_pixel_rgb(dest_x as u32, dest_y as u32) {
                    // Alpha blending vzorec: Out = Fg * alpha + Bg * (1 - alpha)
                    let out_r = (fg_pixel[0] as f32 * alpha + bg_r as f32 * (1.0 - alpha)) as u8;
                    let out_g = (fg_pixel[1] as f32 * alpha + bg_g as f32 * (1.0 - alpha)) as u8;
                    let out_b = (fg_pixel[2] as f32 * alpha + bg_b as f32 * (1.0 - alpha)) as u8;

                    vram.set_pixel(dest_x as u32, dest_y as u32, out_r, out_g, out_b);
                }
            }
        }
    }
}
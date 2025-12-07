use crate::utils::drawing::draw_line;
use crate::utils::point::{BezierCurve, Point};
use crate::vram::VRam;

fn draw_point_circle(vram: &mut VRam, center: Point, radius: u32, r: u8, g: u8, b: u8) {
    let cx = center.x.round() as i32;
    let cy = center.y.round() as i32;
    let r_sq = radius * radius;

    for y in (cy - radius as i32)..=(cy + radius as i32) {
        for x in (cx - radius as i32)..=(cx + radius as i32) {
            let dx = x - cx;
            let dy = y - cy;

            if (dx * dx) as u32 + (dy * dy) as u32 <= r_sq {
                if x >= 0 && x < vram.width as i32 && y >= 0 && y < vram.height as i32 {
                    vram.set_pixel(x as u32, y as u32, r, g, b);
                }
            }
        }
    }
}


pub fn draw_bezier_spline(vram: &mut VRam, points: &[Point], d: f32) {
    if points.len() < 3 {
        eprintln!("Zadání vyžaduje alespoň 3 body.");
        return;
    }

    for p in points.iter() {
        draw_point_circle(vram, *p, 5, 255, 0, 0);
    }

    let mut p_ext: Vec<Point> = Vec::with_capacity(points.len() + 2);
    p_ext.push(points[0]);
    p_ext.extend_from_slice(points);
    p_ext.push(points[points.len() - 1]);

    let n = points.len();
    let mut l_points: Vec<Point> = Vec::with_capacity(n + 1);
    let mut r_points: Vec<Point> = Vec::with_capacity(n + 1);

    for i in 1..=n {
        let p_i = p_ext[i];
        let v = p_ext[i + 1].sub(&p_ext[i - 1]);
        let l_i = p_i.sub(&v.scale(1.0 / 6.0));
        let r_i = p_i.add(&v.scale(1.0 / 6.0));

        l_points.push(l_i);
        r_points.push(r_i);
    }

    for i in 1..n {
        let curve = BezierCurve {
            p0: points[i-1],     // P_i
            p1: r_points[i-1],   // R_i
            p2: l_points[i],     // L_{i+1}
            p3: points[i],       // P_{i+1}
        };

        let mut prev_point = curve.p0;
        let mut t = d;

        while t <= 1.0 {
            let next_point = curve.evaluate(t);
            draw_line(vram, prev_point, next_point, 25, 255, 25);
            prev_point = next_point;
            t += d;
        }

        draw_line(vram, prev_point, curve.p3, 25, 255, 25);
    }
}
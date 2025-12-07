use crate::utils::point::{BezierCurve, Point};

impl BezierCurve {
    pub fn evaluate(&self, t: f32) -> Point {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        let b0 = mt3;                  // (1-t)^3
        let b1 = 3.0 * mt2 * t;        // 3(1-t)^2 t
        let b2 = 3.0 * mt * t2;        // 3(1-t) t^2
        let b3 = t3;                   // t^3

        let x = self.p0.x * b0 + self.p1.x * b1 + self.p2.x * b2 + self.p3.x * b3;
        let y = self.p0.y * b0 + self.p1.y * b1 + self.p2.y * b2 + self.p3.y * b3;

        Point::new(x, y)
    }
}
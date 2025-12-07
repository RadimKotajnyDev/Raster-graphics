#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn scale(&self, s: f32) -> Point {
        Point::new(self.x * s, self.y * s)
    }
}

#[derive(Clone, Debug)]
pub struct BezierCurve {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}
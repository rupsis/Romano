#[derive(Default, Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub type Vector = Tuple;
pub type Point = Tuple;

pub const fn vector(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z, w: 0.0 }
}

pub const fn point(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, w: 1.0 }
}

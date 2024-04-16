use std::ops::{Add, Sub, Neg};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

const EPSILON: f64 = 0.00001;

pub type Vector = Tuple;
pub type Point = Tuple;

pub const fn vector(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z, w: 0.0 }
}

pub const fn point(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, w: 1.0 }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x, 
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

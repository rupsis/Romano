use std::ops::{Add, Mul, Sub};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub const fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

/*  Operator overloading */

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

/* Method implementations */

fn clamp(n: u32) -> u32 {
    n.max(0).min(255)
}

fn to_255(color_channel: f64) -> u32 {
    clamp((color_channel * 255.0).round() as u32)
}

impl Color {
    // pub fn magnitude(self) -> f64 {
    //     (self.r.powi(2) + self.g.powi(2) + self.b.powi(2)).sqrt()
    // }

    // pub fn normalize(self) -> Self {
    //     let mag: f64 = self.magnitude();
    //     Self {
    //         r: self.r / mag,
    //         g: self.g / mag,
    //         b: self.b / mag,
    //     }
    // }

    pub fn write_color(self) -> String {
        format!("{} {} {}\n", to_255(self.r), to_255(self.g), to_255(self.b))
    }
}

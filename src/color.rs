use std::ops::{Add, Mul, Sub};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn clip_color(color_channel: f64) -> f64 {
    if color_channel < 0.0 {
        return 0.0;
    }
    if color_channel > 255.0 {
        return 255.0;
    }
    color_channel
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color {
        r: clip_color(r),
        g: clip_color(g),
        b: clip_color(b),
    }
}

/*  Operator overloading */

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        color(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        color(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        color(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        color(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

/* Method implementations */

impl Color {
    pub fn magnitude(self) -> f64 {
        (self.r.powi(2) + self.g.powi(2) + self.b.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag: f64 = self.magnitude();
        color(self.r / mag, self.g / mag, self.b / mag)
    }
}

use crate::color::{color, Color};

#[derive(Debug, Default)]
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub height: u32,
    pub width: u32,
}

pub fn canvas(width: u32, height: u32) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![color(0.0, 0.0, 0.0); (width * height) as usize],
    }
}

impl Canvas {
    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) -> &mut Self {
        self.pixels[(x * y) as usize] = color;
        self
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        self.pixels[(x * y) as usize]
    }
}
use crate::color::{color, Color};
use std::fs::File;
use std::io::Write;

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
        self.pixels[((self.width * y) + x) as usize] = color;
        self
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        self.pixels[((self.width * y) + x) as usize]
    }

    /* opting to write single pixel lines rather than
     *  manage 70 character lines. Images will be converted
     *  to png (etc) anyways.
     */
    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);

        let mut pixels: String = String::new();

        for pixel in self.pixels.iter() {
            pixels.push_str(&pixel.write_color());
        }

        format!("{}\n{}", header, pixels)
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let filename = path;
        let content = self.to_ppm();
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())
    }
}

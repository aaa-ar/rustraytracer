use crate::vec3::Vec3;
use std::io::{self, Write};
use std::ops;
use std::ops::DivAssign;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Vec3> for Color {
    fn from(col: Vec3) -> Color {
        Color {
            r: (255.99 * col.x) as u8,
            g: (255.99 * col.y) as u8,
            b: (255.99 * col.z) as u8,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        *self = Color {
            r: (f32::from(self.r) / other) as u8,
            g: (f32::from(self.g) / other) as u8,
            b: (f32::from(self.b) / other) as u8,
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image::with_background(width, height, Color { r: 0, g: 0, b: 0 })
    }

    pub fn with_background(width: u32, height: u32, color: Color) -> Image {
        Image {
            width,
            height,
            pixels: vec![color; (width * height) as usize],
        }
    }

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }

    pub fn write_ppm(self, handle: &mut Write) -> io::Result<i32> {
        let magic_number = "P3";
        let max_color = 255u8;

        writeln!(handle, "{}", magic_number)?;
        writeln!(handle, "{} {}", self.width, self.height)?;
        writeln!(handle, "{}", max_color)?;

        for pixel in &self.pixels {
            let Color { r, g, b } = pixel;
            writeln!(handle, "{} {} {}", r, g, b)?;
        }

        Ok(0)
    }

    fn index_of(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }
}

impl ops::Index<(u32, u32)> for Image {
    type Output = Color;

    fn index(&self, (x, y): (u32, u32)) -> &Color {
        let index = self.index_of(x, y);
        &self.pixels[index]
    }
}

impl ops::IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Color {
        let index = self.index_of(x, y);
        &mut self.pixels[index]
    }
}

use crate::canvas::Canvas;
use crate::color::Color;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use std::fs::File;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn from_u8(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }

    pub fn from_f32(r: f32, g: f32, b: f32) -> RGB {
        RGB::from_u8(Self::to_u8(r), Self::to_u8(g), Self::to_u8(b))
    }

    pub fn from_color(c: Color) -> RGB {
        RGB::from_f32(c.red, c.green, c.blue)
    }

    pub fn black() -> RGB {
        RGB::from_u8(0, 0, 0)
    }

    fn to_u8(f: f32) -> u8 {
        (f.clamp(0., 1.) * 255.).round() as u8
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<RGB>,
}

impl Image {
    pub fn new(width: usize, height: usize, bg_color: RGB) -> Image {
        Image {
            width,
            height,
            data: vec![bg_color; width * height],
        }
    }

    pub fn from_canvas(canvas: Canvas) -> Image {
        let mut img = Image::new(canvas.width, canvas.height, RGB::black());
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                img.set(x, y, RGB::from_color(canvas.get(x, y)));
            }
        }
        img
    }

    pub fn set(&mut self, x: usize, y: usize, rgb: RGB) {
        self.data[y * self.width + x] = rgb;
    }

    pub fn get(&self, x: usize, y: usize) -> RGB {
        self.data[y * self.width + x]
    }

    pub fn write_png(&self, filename: &str) {
        let data = {
            let mut data: Vec<u8> = Vec::with_capacity(self.width * self.height * 3);
            for rgb in &self.data {
                data.push(rgb.r);
                data.push(rgb.g);
                data.push(rgb.b);
            }
            data
        };

        let out = File::create(filename).unwrap();
        let encoder = PngEncoder::new(out);
        encoder
            .write_image(
                &data,
                self.width as u32,
                self.height as u32,
                ColorType::Rgb8,
            )
            .unwrap();
    }

    pub fn to_ppm(&self) -> Vec<String> {
        let mut ppm: Vec<String> = Vec::with_capacity(3 + self.height);
        ppm.push("P3".to_string());
        ppm.push(format!("{} {}", self.width, self.height));
        ppm.push("255".to_string());
        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let rgb = self.get(x, y);
                row += &format!("{} {} {} ", rgb.r, rgb.g, rgb.b);
            }
            ppm.push(row);
        }
        ppm
    }
}

#[cfg(test)]
#[path = "./image_test.rs"]
mod tests;

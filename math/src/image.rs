use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use std::fs::File;

pub struct Image {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(rows: usize, cols: usize) -> Image {
        Image {
            rows,
            cols,
            data: vec![0; rows * cols * 3],
        }
    }

    pub fn write(&self, filename: &str) {
        let out = File::create(filename).unwrap();
        let encoder = PngEncoder::new(out);
        encoder
            .write_image(
                &self.data,
                self.cols as u32,
                self.rows as u32,
                ColorType::Rgb8,
            )
            .unwrap();
    }
}

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, bg_color: Color) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![bg_color; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.data[y * self.width + x] = c;
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }
}

#[cfg(test)]
#[path = "./canvas_test.rs"]
mod tests;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { width, height }
    }
}

#[cfg(test)]
#[path = "./canvas_test.rs"]
mod tests;

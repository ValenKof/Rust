#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, c: Color) -> Self::Output {
        Self {
            red: self.red + c.red,
            green: self.green + c.green,
            blue: self.blue + c.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, c: Color) -> Self::Output {
        Self {
            red: self.red - c.red,
            green: self.green - c.green,
            blue: self.blue - c.blue,
        }
    }
}
impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, c: Color) -> Self::Output {
        Self {
            red: self.red * c.red,
            green: self.green * c.green,
            blue: self.blue * c.blue,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, c: f32) -> Self::Output {
        Self {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, c: Color) -> Self::Output {
        c * self
    }
}

pub fn hadamard_product(a: Color, b: Color) -> Color {
    Color {
        red: a.red * b.red,
        green: a.green * b.green,
        blue: a.blue * b.blue,
    }
}

impl crate::approx::Approx for Color {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        self.red.is_near(&other.red, eps)
            && self.green.is_near(&other.green, eps)
            && self.blue.is_near(&other.blue, eps)
    }
}

#[cfg(test)]
#[path = "./color_test.rs"]
mod tests;

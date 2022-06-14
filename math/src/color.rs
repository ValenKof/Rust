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

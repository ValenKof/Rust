#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, t: Self) -> Self::Output {
        Self {
            x: self.x + t.x,
            y: self.y + t.y,
            z: self.z + t.z,
            w: self.w + t.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, t: Self) -> Self::Output {
        Self {
            x: self.x - t.x,
            y: self.y - t.y,
            z: self.z - t.z,
            w: self.w - t.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, f: f32) -> Self::Output {
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f,
        }
    }
}

impl std::ops::Mul<Tuple> for f32 {
    type Output = Tuple;

    fn mul(self, t: Tuple) -> Self::Output {
        t * self
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, f: f32) -> Self::Output {
        self * (1.0 / f)
    }
}

#[cfg(test)]
#[path = "./tuple_test.rs"]
mod tests;

use crate::point::Point;
use crate::vector::Vector;

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

    pub fn sq_len(self) -> f32 {
        dot(self, self)
    }

    pub fn len(self) -> f32 {
        self.sq_len().sqrt()
    }

    pub fn normalized(self) -> Tuple {
        self / self.len()
    }

    pub fn reflected(self, normal: Tuple) -> Tuple {
        self - normal * (2. * dot(self, normal))
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

pub fn dot(a: Tuple, b: Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Tuple, b: Tuple) -> Tuple {
    Tuple::vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

impl crate::approx::Approx for Tuple {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        self.x.is_near(&other.x, eps)
            && self.y.is_near(&other.y, eps)
            && self.z.is_near(&other.z, eps)
            && self.w.is_near(&other.w, eps)
    }
}

impl std::convert::From<Vector> for Tuple {
    fn from(v: Vector) -> Tuple {
        Tuple::vector(v.x, v.y, v.z)
    }
}

impl std::convert::TryFrom<Tuple> for Vector {
    type Error = &'static str;

    fn try_from(t: Tuple) -> Result<Vector, Self::Error> {
        if t.is_vector() {
            Ok(Vector::new(t.x, t.y, t.z))
        } else {
            Err("Tuple has w != 0")
        }
    }
}

impl std::convert::From<Point> for Tuple {
    fn from(p: Point) -> Tuple {
        Tuple::point(p.x, p.y, p.z)
    }
}

impl std::convert::TryFrom<Tuple> for Point {
    type Error = &'static str;

    fn try_from(t: Tuple) -> Result<Point, Self::Error> {
        if t.is_point() {
            Ok(Point::new(t.x, t.y, t.z))
        } else {
            Err("Tuple has w != 1")
        }
    }
}

pub fn tuple<T: crate::F32Const>(x: T, y: T, z: T, w: T) -> Tuple {
    Tuple::new(x.to_f32(), y.to_f32(), z.to_f32(), w.to_f32())
}

#[cfg(test)]
#[path = "./tuple_test.rs"]
mod tests;

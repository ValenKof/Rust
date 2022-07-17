pub mod approx;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod image;
pub mod intersect;
pub mod light;
pub mod lighting;
pub mod material;
pub mod matrix;
pub mod point;
pub mod ray;
pub mod sphere;
pub mod transforms;
pub mod tuple;
pub mod vector;
pub mod view;
pub mod world;

pub trait F32Const {
    fn to_f32(self) -> f32;
}

impl F32Const for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl F32Const for i16 {
    fn to_f32(self) -> f32 {
        self.into()
    }
}

#[cfg(test)]
mod test_utils;

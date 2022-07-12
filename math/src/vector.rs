#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
}

pub fn vector<T: crate::F32Const>(x: T, y: T, z: T) -> crate::tuple::Tuple {
    crate::tuple::Tuple::vector(x.to_f32(), y.to_f32(), z.to_f32())
}

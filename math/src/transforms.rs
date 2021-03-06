use crate::matrix::Matrix;
use crate::point::Point;
use crate::tuple::Tuple;
use crate::vector::Vector;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    Matrix::new([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ])
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    Matrix::diagonal([x, y, z, 1.])
}

pub fn rotation_x(rad: f32) -> Matrix<4, 4> {
    let (sin, cos) = rad.sin_cos();
    Matrix::new([
        [1., 0., 0., 0.],
        [0., cos, -sin, 0.],
        [0., sin, cos, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotation_y(rad: f32) -> Matrix<4, 4> {
    let (sin, cos) = rad.sin_cos();
    Matrix::new([
        [cos, 0., sin, 0.],
        [0., 1., 0., 0.],
        [-sin, 0., cos, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotation_z(rad: f32) -> Matrix<4, 4> {
    let (sin, cos) = rad.sin_cos();
    Matrix::new([
        [cos, -sin, 0., 0.],
        [sin, cos, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix<4, 4> {
    Matrix::new([
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub trait Transform: Sized {
    fn apply(&self, t: &Matrix<4, 4>) -> Self;

    fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        self.apply(&translation(x, y, z))
    }
    fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        self.apply(&scaling(x, y, z))
    }
    fn uniform_scale(&self, f: f32) -> Self {
        self.apply(&scaling(f, f, f))
    }
    fn rotate_x(&self, rad: f32) -> Self {
        self.apply(&rotation_x(rad))
    }
    fn rotate_y(&self, rad: f32) -> Self {
        self.apply(&rotation_y(rad))
    }
    fn rotate_z(&self, rad: f32) -> Self {
        self.apply(&rotation_z(rad))
    }
    fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        self.apply(&shearing(xy, xz, yx, yz, zx, zy))
    }
}

impl Transform for Matrix<4, 4> {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        t * self
    }
}

impl Transform for Tuple {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        t * *self
    }
}

impl Transform for Vector {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        Vector::try_from(t * Tuple::from(*self)).unwrap()
    }
}

impl Transform for Point {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        Point::try_from(t * Tuple::from(*self)).unwrap()
    }
}

#[cfg(test)]
#[path = "./transforms_test.rs"]
mod tests;

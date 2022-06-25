use crate::matrix::Matrix;

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

#[cfg(test)]
#[path = "./transforms_test.rs"]
mod tests;

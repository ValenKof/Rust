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

#[cfg(test)]
#[path = "./transforms_test.rs"]
mod tests;

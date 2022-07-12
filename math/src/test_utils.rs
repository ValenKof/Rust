pub use crate::approx::Approx;
pub use crate::assert_near;

use crate::matrix::Matrix;

#[macro_export]
macro_rules! assert_near {
    ($lhs:expr, $rhs:expr) => {
        assert_near!($lhs, $rhs, 1e-5)
    };
    ($lhs:expr, $rhs:expr, $eps:expr) => {
        assert!((&$lhs).is_near(&$rhs, $eps))
    };
}

pub fn matrix<const N: usize, const M: usize>(data: [[i16; M]; N]) -> Matrix<N, M> {
    let mut vec = vec![vec![0_f32; M]; N];
    for row in 0..N {
        for col in 0..M {
            vec[row][col] = data[row][col].into();
        }
    }
    Matrix::from_vec(vec)
}

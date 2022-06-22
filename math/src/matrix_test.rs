use super::Matrix;
use crate::approx::Approx;
use crate::assert_near;
use crate::tuple::Tuple;

#[test]
fn test_constructing_4x4_matrix() {
    let m: Matrix<4, 4> = Matrix::new([
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ]);
    assert_eq!(m[(0, 0)], 1.);
    assert_eq!(m[(0, 3)], 4.);
    assert_eq!(m[(1, 0)], 5.5);
    assert_eq!(m[(1, 2)], 7.5);
    assert_eq!(m[(2, 2)], 11.);
    assert_eq!(m[(3, 0)], 13.5);
    assert_eq!(m[(3, 2)], 15.5);
}

#[test]
fn test_constructing_2x2_matrix() {
    let m: Matrix<2, 2> = Matrix::new([[-3., 5.], [1., -2.]]);
    assert_eq!(m[(0, 0)], -3.);
    assert_eq!(m[(0, 1)], 5.);
    assert_eq!(m[(1, 0)], 1.);
    assert_eq!(m[(1, 1)], -2.);
}

#[test]
fn test_constructing_3x3_matrix() {
    let m: Matrix<3, 3> = Matrix::new([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);
    assert_eq!(m[(0, 0)], -3.);
    assert_eq!(m[(1, 1)], -2.);
    assert_eq!(m[(2, 2)], 1.);
}

#[test]
fn test_matrix_equality_with_identical_matrices() {
    let a: Matrix<2, 3> = Matrix::new([[-3., 5., -6.], [1., -2., -6.]]);
    let b: Matrix<2, 3> = Matrix::new([[-3., 5., -6.], [1., -2., -6.]]);
    assert_eq!(a, b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    let a: Matrix<2, 3> = Matrix::new([[-3., 5., -6.], [1., -2., -6.]]);
    let b: Matrix<2, 3> = Matrix::new([[-3., 5., -6.], [1., -2., -6.1]]);
    assert_ne!(a, b);
}

#[test]
fn test_multiply_matrices() {
    let a: Matrix<2, 3> = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
    let b: Matrix<3, 2> = Matrix::new([[7., 8.], [9., 10.], [11., 12.]]);
    let c: Matrix<2, 2> = Matrix::new([[58., 64.], [139., 154.]]);
    assert_eq!(&a * &b, c);
}

#[test]
fn test_multiply_matrix_by_tuple() {
    let a: Matrix<4, 4> = Matrix::new([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]);
    let b = Tuple::new(1., 2., 3., 1.);
    assert_eq!(
        (&a * &b.to_matrix()).to_tuple(),
        Tuple::new(18., 24., 33., 1.)
    );
}

#[test]
fn test_multiply_tuple_by_matrix() {
    let a = Tuple::new(1., 2., 3., 1.);
    let b: Matrix<4, 4> = Matrix::new([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]);
    assert_eq!(
        (&a.to_matrix().transpose() * &b).transpose().to_tuple(),
        Tuple::new(29., 28., 23., 12.)
    );
}

#[test]
fn test_create_zero_matrix() {
    let a: Matrix<3, 2> = Matrix::zeroes();
    let b = Matrix::new([[0., 0.], [0., 0.], [0., 0.]]);
    assert_eq!(a, b);
}

#[test]
fn test_create_identity_matrix() {
    let a: Matrix<3, 3> = Matrix::identity();
    let b = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    assert_eq!(a, b);
}

#[test]
fn test_multiply_matrix_by_identity() {
    let a: Matrix<4, 4> = Matrix::new([
        [0., 1., 2., 4.],
        [1., 2., 4., 8.],
        [2., 4., 8., 16.],
        [4., 8., 16., 32.],
    ]);
    assert_eq!(&a * &Matrix::identity(), a);
}

#[test]
fn test_transpose_matrix() {
    let a: Matrix<4, 4> = Matrix::new([
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    ]);
    let b: Matrix<4, 4> = Matrix::new([
        [0., 9., 1., 0.],
        [9., 8., 8., 0.],
        [3., 0., 5., 5.],
        [0., 8., 3., 8.],
    ]);
    assert_eq!(a.transpose(), b);
}

#[test]
fn test_determinant_of_2x2_matrix() {
    let a = Matrix::new([[1., 5.], [-3., 2.]]);
    assert_eq!(a.determinant(), 17.);
}

#[test]
fn test_determinant_of_3x3_matrix() {
    let a = Matrix::new([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);
    assert_eq!(a.determinant(), -196.);
}

#[test]
fn test_determinant_of_4x4_matrix() {
    let a = Matrix::new([
        [-2., -8., 3., 5.],
        [-3., 1., 7., 3.],
        [1., 2., -9., 6.],
        [-6., 7., 7., -9.],
    ]);
    assert_eq!(a.determinant(), -4071.);
}

#[test]
fn test_determinant_of_invertible_matrix() {
    let a = Matrix::new([
        [6., 4., 4., 4.],
        [5., 5., 7., 6.],
        [4., -9., 3., -7.],
        [9., 1., 7., -6.],
    ]);
    assert_near!(a.determinant(), -2120., 1e-3);
    assert!(a.inverse().is_some());
}

#[test]
fn test_determinant_of_non_invertible_matrix() {
    let a = Matrix::new([
        [-4., 2., -2., -3.],
        [9., 6., 2., 6.],
        [0., -5., 1., -5.],
        [0., 0., 0., 0.],
    ]);
    assert_eq!(a.determinant(), 0.0);
    assert!(a.inverse().is_none());
}

use super::Matrix;

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

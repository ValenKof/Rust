use super::{scaling, translation};
//use crate::matrix::Matrix;
use crate::tuple::Tuple;
//use crate::assert_near;

#[test]
fn test_multiply_by_translation_matrix() {
    let transform = translation(5., -3., 2.);
    let p = Tuple::point(-3., 4., 5.);
    assert_eq!(&transform * p, Tuple::point(2., 1., 7.));
}

#[test]
fn test_multiply_by_inverse_of_translation_matrix() {
    let transform = translation(5., -3., 2.);
    let inv = transform.inverse().unwrap();
    let p = Tuple::point(-3., 4., 5.);
    assert_eq!(&inv * p, Tuple::point(-8., 7., 3.));
}

#[test]
fn test_translation_does_not_affect_vectors() {
    let transform = translation(5., -3., 2.);
    let v = Tuple::vector(-3., 4., 5.);
    assert_eq!(&transform * v, v);
}

#[test]
fn test_apply_scaling_to_point() {
    let transform = scaling(2., 3., 4.);
    let p = Tuple::point(-4., 6., 8.);
    assert_eq!(&transform * p, Tuple::point(-8., 18., 32.));
}

#[test]
fn test_apply_scaling_to_vector() {
    let transform = scaling(2., 3., 4.);
    let v = Tuple::vector(-4., 6., 8.);
    assert_eq!(&transform * v, Tuple::vector(-8., 18., 32.));
}

#[test]
fn test_multiply_by_inverse_of_scaling_matrix() {
    let transform = scaling(2., 3., 4.);
    let inv = transform.inverse().unwrap();
    let v = Tuple::vector(-4., 6., 8.);
    assert_eq!(&inv * v, Tuple::vector(-2., 2., 2.));
}

#[test]
fn test_reflection_by_negative_scaling() {
    let transform = scaling(-1., 1., 1.);
    let p = Tuple::point(2., 3., 4.);
    assert_eq!(&transform * p, Tuple::point(-2., 3., 4.));
}

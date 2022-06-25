use super::*;
use crate::test_utils::*;
use crate::tuple::Tuple;
use std::f32::consts::{FRAC_1_SQRT_2, PI};

#[test]
fn test_multiply_by_translation_matrix() {
    let transform = translation(5., -3., 2.);
    let p = point(-3, 4, 5);
    assert_eq!(&transform * p, point(2, 1, 7));
}

#[test]
fn test_multiply_by_inverse_of_translation_matrix() {
    let transform = translation(5., -3., 2.);
    let inv = transform.inverse().unwrap();
    let p = point(-3, 4, 5);
    assert_eq!(&inv * p, point(-8, 7, 3));
}

#[test]
fn test_translation_does_not_affect_vectors() {
    let transform = translation(5., -3., 2.);
    let v = vector(-3, 4, 5);
    assert_eq!(&transform * v, v);
}

#[test]
fn test_apply_scaling_to_point() {
    let transform = scaling(2., 3., 4.);
    let p = point(-4, 6, 8);
    assert_eq!(&transform * p, point(-8, 18, 32));
}

#[test]
fn test_apply_scaling_to_vector() {
    let transform = scaling(2., 3., 4.);
    let v = vector(-4, 6, 8);
    assert_eq!(&transform * v, vector(-8, 18, 32));
}

#[test]
fn test_multiply_by_inverse_of_scaling_matrix() {
    let transform = scaling(2., 3., 4.);
    let inv = transform.inverse().unwrap();
    let v = vector(-4, 6, 8);
    assert_eq!(&inv * v, vector(-2, 2, 2));
}

#[test]
fn test_reflection_by_negative_scaling() {
    let transform = scaling(-1., 1., 1.);
    let p = point(2, 3, 4);
    assert_eq!(&transform * p, point(-2, 3, 4));
}

#[test]
fn test_rotate_point_around_x_axis() {
    let p = point(0, 1, 0);
    let half_quarter = rotation_x(PI / 4.);
    let full_quarter = rotation_x(PI / 2.);
    assert_near!(
        &half_quarter * p,
        Tuple::point(0., FRAC_1_SQRT_2, FRAC_1_SQRT_2)
    );
    assert_near!(&full_quarter * p, point(0, 0, 1));
}

#[test]
fn test_rotate_point_around_y_axis() {
    let p = point(0, 0, 1);
    let half_quarter = rotation_y(PI / 4.);
    let full_quarter = rotation_y(PI / 2.);
    assert_near!(
        &half_quarter * p,
        Tuple::point(FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2)
    );
    assert_near!(&full_quarter * p, point(1, 0, 0));
}

#[test]
fn test_rotate_point_around_z_axis() {
    let p = point(0, 1, 0);
    let half_quarter = rotation_z(PI / 4.);
    let full_quarter = rotation_z(PI / 2.);
    assert_near!(
        &half_quarter * p,
        Tuple::point(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.)
    );
    assert_near!(&full_quarter * p, point(-1, 0, 0));
}

#[test]
fn test_shearing() {
    let t0 = shearing(1., 0., 0., 0., 0., 0.);
    let t1 = shearing(0., 1., 0., 0., 0., 0.);
    let t2 = shearing(0., 0., 1., 0., 0., 0.);
    let t3 = shearing(0., 0., 0., 1., 0., 0.);
    let t4 = shearing(0., 0., 0., 0., 1., 0.);
    let t5 = shearing(0., 0., 0., 0., 0., 1.);
    let p = point(2, 3, 4);
    assert_eq!(&t0 * p, point(5, 3, 4));
    assert_eq!(&t1 * p, point(6, 3, 4));
    assert_eq!(&t2 * p, point(2, 5, 4));
    assert_eq!(&t3 * p, point(2, 7, 4));
    assert_eq!(&t4 * p, point(2, 3, 6));
    assert_eq!(&t5 * p, point(2, 3, 7));
}

#[test]
fn test_individual_transformations() {
    let p1 = point(1, 0, 1);
    let a = rotation_x(PI / 2.);
    let b = scaling(5., 5., 5.);
    let c = translation(10., 5., 7.);
    let p2 = &a * p1;
    assert_near!(p2, point(1, -1, 0));
    let p3 = &b * p2;
    assert_near!(p3, point(5, -5, 0));
    let p4 = &c * p3;
    assert_near!(p4, point(15, 0, 7));
}

#[test]
fn test_chained_transformations() {
    let p = point(1, 0, 1);
    let a = rotation_x(PI / 2.);
    let b = scaling(5., 5., 5.);
    let c = translation(10., 5., 7.);
    let t = &(&c * &b) * &a;
    assert_near!(&t * p, point(15, 0, 7));
}

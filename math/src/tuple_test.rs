use super::*;
use crate::test_utils::*;
use std::f32::consts::FRAC_1_SQRT_2;

#[test]
fn test_create_tuple_with_w_equal_to_1() {
    let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(t.x, 4.3);
    assert_eq!(t.y, -4.2);
    assert_eq!(t.z, 3.1);
    assert_eq!(t.w, 1.0);
    assert!(t.is_point());
    assert!(!t.is_vector());
}

#[test]
fn test_create_tuple_with_w_equal_to_0() {
    let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(t.x, 4.3);
    assert_eq!(t.y, -4.2);
    assert_eq!(t.z, 3.1);
    assert_eq!(t.w, 0.0);
    assert!(!t.is_point());
    assert!(t.is_vector());
}

#[test]
fn test_create_point() {
    let p = point(4, -4, 3);
    assert_eq!(p, tuple(4, -4, 3, 1));
}

#[test]
fn test_create_vector() {
    let v = vector(4, -4, 3);
    assert_eq!(v, tuple(4, -4, 3, 0));
}

#[test]
fn test_adding_two_tuples() {
    let t1 = tuple(3, -2, 5, 1);
    let t2 = tuple(-2, 3, 1, 0);
    assert_eq!(t1 + t2, tuple(1, 1, 6, 1));
}

#[test]
fn test_subtracting_two_points() {
    let p1 = point(3, 2, 1);
    let p2 = point(5, 6, 7);
    assert_eq!(p1 - p2, vector(-2, -4, -6));
}

#[test]
fn test_subtracting_vector_from_point() {
    let p = point(3, 2, 1);
    let v = vector(5, 6, 7);
    assert_eq!(p - v, point(-2, -4, -6));
}

#[test]
fn test_subtracting_two_vectors() {
    let v1 = vector(3, 2, 1);
    let v2 = vector(5, 6, 7);
    assert_eq!(v1 - v2, vector(-2, -4, -6));
}

#[test]
fn test_subtracting_vector_from_zero_vector() {
    let zero = vector(0, 0, 0);
    let v = vector(1, 2, 3);
    assert_eq!(zero - v, vector(-1, -2, -3));
}

#[test]
fn test_negating_tuple() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(-t, tuple(-1, 2, -3, 4));
}

#[test]
fn test_multiplying_tuple_by_scalar() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
    assert_eq!(3.5 * t, Tuple::new(3.5, -7., 10.5, -14.));
}

#[test]
fn test_multiplying_tuple_by_fraction() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_dividing_tuple_by_scalar() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t / 2., Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_length_of_vector() {
    assert_eq!(vector(1, 0, 0).len(), 1.0);
    assert_eq!(vector(0, 1, 0).len(), 1.0);
    assert_eq!(vector(0, 0, 1).len(), 1.0);
    assert_eq!(vector(1, 2, 3).len(), 14_f32.sqrt());
    assert_eq!(vector(-1, -2, -3).len(), 14_f32.sqrt());
}

#[test]
fn test_normalize_vector() {
    assert_eq!(vector(4, 0, 0).normalize(), vector(1, 0, 0));
    assert_near!(
        vector(1, 2, 3).normalize(),
        Tuple::vector(0.26726, 0.53452, 0.80178)
    );
    assert_near!(vector(1, 2, 3).normalize().len(), 1.0);
}

#[test]
fn test_dot_product_of_two_vectors() {
    let a = vector(1, 2, 3);
    let b = vector(2, 3, 4);
    assert_eq!(dot(a, b), 20.0);
}

#[test]
fn test_cross_product_of_two_vector() {
    let a = vector(1, 2, 3);
    let b = vector(2, 3, 4);
    assert_eq!(cross(a, b), vector(-1, 2, -1));
    assert_eq!(cross(b, a), vector(1, -2, 1));
}

#[test]
fn test_tuple_from_point() {
    let p = Point::new(4.3, -4.2, 3.1);
    assert_eq!(p.x, 4.3);
    assert_eq!(p.y, -4.2);
    assert_eq!(p.z, 3.1);
    assert_eq!(Tuple::from(p), Tuple::new(4.3, -4.2, 3.1, 1.0));
}

#[test]
fn test_point_from_tuple() {
    let t0 = tuple(4, 3, 2, 0);
    let t1 = tuple(4, 3, 2, 1);
    assert_eq!(Point::try_from(t0), Err("Tuple has w != 1"));
    assert_eq!(Point::try_from(t1), Ok(Point::new(4.0, 3.0, 2.0)));
}

#[test]
fn test_reflect_vector_at_45_angle() {
    let v = vector(1, -1, 0);
    let n = vector(0, 1, 0);
    assert_eq!(v.reflect(n), vector(1, 1, 0));
}

#[test]
fn test_reflect_vector_off_slanted_surface() {
    let v = vector(0, -1, 0);
    let n = Tuple::vector(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
    assert_near!(v.reflect(n), vector(1, 0, 0));
}

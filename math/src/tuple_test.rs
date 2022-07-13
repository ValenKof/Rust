use super::*;
use crate::point::point;
//use crate::test_utils::*;
use crate::vector::vector;

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
fn test_adding_two_tuples() {
    let t1 = tuple(3, -2, 5, 1);
    let t2 = tuple(-2, 3, 1, 0);
    assert_eq!(t1 + t2, tuple(1, 1, 6, 1));
}

#[test]
fn test_negating_tuple() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(-t, tuple(-1, 2, -3, 4));
}

#[test]
fn test_multiplying_tuple_by_scalar() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t * 3.5, tuple(3.5, -7., 10.5, -14.));
    assert_eq!(3.5 * t, tuple(3.5, -7., 10.5, -14.));
}

#[test]
fn test_multiplying_tuple_by_fraction() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_dividing_tuple_by_scalar() {
    let t = tuple(1, -2, 3, -4);
    assert_eq!(t / 2., tuple(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_vector_struct() {
    let v = vector(4.3, -4.2, 3.1);
    assert_eq!(v.x, 4.3);
    assert_eq!(v.y, -4.2);
    assert_eq!(v.z, 3.1);
    assert_eq!(Tuple::from(v), Tuple::new(4.3, -4.2, 3.1, 0.0));
}

#[test]
fn test_vector_from_tuple() {
    let t0 = tuple(4, 3, 2, 0);
    let t1 = tuple(4, 3, 2, 1);
    assert_eq!(Vector::try_from(t0), Ok(Vector::new(4.0, 3.0, 2.0)));
    assert_eq!(Vector::try_from(t1), Err("Tuple has w != 0"));
}

#[test]
fn test_tuple_from_point() {
    let p = point(4.3, -4.2, 3.1);
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

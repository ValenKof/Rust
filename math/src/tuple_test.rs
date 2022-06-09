use super::Tuple;

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
    let p = Tuple::point(4., -4., 3.);
    assert_eq!(p, Tuple::new(4., -4., 3., 1.));
}

#[test]
fn test_create_vector() {
    let v = Tuple::vector(4., -4., 3.);
    assert_eq!(v, Tuple::new(4., -4., 3., 0.));
}

#[test]
fn test_adding_two_tuples() {
    let t1 = Tuple::new(3., -2., 5., 1.);
    let t2 = Tuple::new(-2., 3., 1., 0.);
    assert_eq!(t1 + t2, Tuple::new(1., 1., 6., 1.));
}

#[test]
fn test_subtracting_two_points() {
    let p1 = Tuple::point(3., 2., 1.);
    let p2 = Tuple::point(5., 6., 7.);
    assert_eq!(p1 - p2, Tuple::vector(-2., -4., -6.));
}

#[test]
fn test_subtracting_vector_from_point() {
    let p = Tuple::point(3., 2., 1.);
    let v = Tuple::vector(5., 6., 7.);
    assert_eq!(p - v, Tuple::point(-2., -4., -6.));
}

#[test]
fn test_subtracting_two_vectors() {
    let v1 = Tuple::vector(3., 2., 1.);
    let v2 = Tuple::vector(5., 6., 7.);
    assert_eq!(v1 - v2, Tuple::vector(-2., -4., -6.));
}

#[test]
fn test_subtracting_vector_from_zero_vector() {
    let zero = Tuple::vector(0., 0., 0.);
    let v = Tuple::vector(1., 2., 3.);
    assert_eq!(zero - v, Tuple::vector(-1., -2., -3.));
}

#[test]
fn test_negating_tuple() {
    let t = Tuple::new(1., -2., 3., -4.);
    assert_eq!(-t, Tuple::new(-1., 2., -3., 4.));
}

#[test]
fn test_multiplying_tuple_by_scalar() {
    let t = Tuple::new(1., -2., 3., -4.);
    assert_eq!(t * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
    assert_eq!(3.5 * t, Tuple::new(3.5, -7., 10.5, -14.));
}

#[test]
fn test_multiplying_tuple_by_fraction() {
    let t = Tuple::new(1., -2., 3., -4.);
    assert_eq!(t * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
}

#[test]
fn test_dividing_tuple_by_scalar() {
    let t = Tuple::new(1., -2., 3., -4.);
    assert_eq!(t / 2., Tuple::new(0.5, -1., 1.5, -2.));
}

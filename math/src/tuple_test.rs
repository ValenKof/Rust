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

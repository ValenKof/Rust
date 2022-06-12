use super::{cross, dot, Tuple};

trait ApproxEq {
    fn is_near(&self, other: &Self, eps: f32) -> bool;
}

impl ApproxEq for f32 {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        (self - other).abs() < eps
    }
}

impl ApproxEq for Tuple {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        self.x.is_near(&other.x, eps)
            && self.y.is_near(&other.y, eps)
            && self.z.is_near(&other.z, eps)
            && self.w.is_near(&other.w, eps)
    }
}

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

#[test]
fn test_length_of_vector() {
    assert_eq!(Tuple::vector(1., 0., 0.).len(), 1.);
    assert_eq!(Tuple::vector(0., 1., 0.).len(), 1.);
    assert_eq!(Tuple::vector(0., 0., 1.).len(), 1.);
    assert_eq!(Tuple::vector(1., 2., 3.).len(), 14_f32.sqrt());
    assert_eq!(Tuple::vector(-1., -2., -3.).len(), 14_f32.sqrt());
}

#[test]
fn test_normalize_vector() {
    assert_eq!(
        Tuple::vector(4., 0., 0.).normalize(),
        Tuple::vector(1., 0., 0.)
    );
    assert!(Tuple::vector(1., 2., 3.)
        .normalize()
        .is_near(&Tuple::vector(0.26726, 0.53452, 0.80178), 1e-5));
    assert!(Tuple::vector(1., 2., 3.)
        .normalize()
        .len()
        .is_near(&1., 1e-5));
}

#[test]
fn test_dot_product_of_two_vectors() {
    let a = Tuple::vector(1., 2., 3.);
    let b = Tuple::vector(2., 3., 4.);
    assert_eq!(dot(&a, &b), 20.);
}

#[test]
fn test_cross_product_of_two_vector() {
    let a = Tuple::vector(1., 2., 3.);
    let b = Tuple::vector(2., 3., 4.);
    assert_eq!(cross(&a, &b), Tuple::vector(-1., 2., -1.));
    assert_eq!(cross(&b, &a), Tuple::vector(1., -2., 1.));
}

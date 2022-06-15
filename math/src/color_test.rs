use super::{hadamard_product, Color};
use crate::approx::Approx;
use crate::assert_near;

#[test]
fn test_create_color() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
}

#[test]
fn test_adding_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_near!(c1 + c2, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn test_subtracting_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_near!(c1 - c2, Color::new(0.2, 0.5, 0.5));
}

#[test]
fn test_multiplying_color_by_scalar() {
    let c1 = Color::new(0.2, 0.3, 0.4);
    let c2 = Color::new(0.4, 0.6, 0.8);
    assert_eq!(c1 * 2.0, c2);
    assert_eq!(2.0 * c1, c2);
}

#[test]
fn test_multiplying_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert_near!(hadamard_product(c1, c2), Color::new(0.9, 0.2, 0.04));
}

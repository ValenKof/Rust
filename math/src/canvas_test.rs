use super::{Canvas, Color};

#[test]
fn test_create_canvas() {
    let c = Canvas::new(10, 20, Color::black());
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn test_writing_to_canvas() {
    let mut c = Canvas::new(10, 20, Color::black());
    let red = Color::new(1., 0., 0.);
    c.set(2, 3, red);
    assert_eq!(c.get(2, 3), red);
}

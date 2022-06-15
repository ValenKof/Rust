use super::Image;
use crate::canvas::Canvas;
use crate::color::Color;

#[test]
fn test_constructing_ppm_header() {
    let c = Canvas::new(5, 3, Color::black());
    assert_eq!(
        Image::from_canvas(c).to_ppm()[0..3],
        vec!["P3", "5 3", "255"]
    );
}

#[test]
fn test_constructing_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3, Color::black());
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    c.set(0, 0, c1);
    c.set(2, 1, c2);
    c.set(4, 2, c3);
    assert_eq!(
        Image::from_canvas(c).to_ppm()[3..6],
        vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 ",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 "
        ]
    );
}

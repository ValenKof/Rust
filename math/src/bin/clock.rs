use math::canvas::Canvas;
use math::color::Color;
use math::image::Image;
use math::point::{point, Point};
use math::transforms::Transform;
use std::f32::consts::PI;

const SIZE: usize = 500;

fn to_screen(p: Point) -> (usize, usize) {
    let x = (p.x as i32).clamp(0, (SIZE - 1) as i32) as usize;
    let y = (p.y as i32).clamp(0, (SIZE - 1) as i32) as usize;
    (x, SIZE - 1 - y)
}

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE, Color::black());
    let three_oclock = point(1.0, 0.0, 0.0);
    for i in 0..12 {
        const HALF: f32 = (SIZE / 2) as f32;
        let point = three_oclock
            .rotate_z(PI * (i as f32) / 6.)
            .uniform_scale(HALF * 0.9)
            .translate(HALF, HALF, 0.0);
        println!("point: {:?}", point);
        let (x, y) = to_screen(point);
        canvas.set(x, y, Color::white());
    }
    Image::from_canvas(canvas).write_png("clock.out.png");
}

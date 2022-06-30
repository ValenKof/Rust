use math::canvas::Canvas;
use math::color::Color;
use math::image::Image;
use math::intersect::{hit, Intersect};
use math::ray::Ray;
use math::sphere::Sphere;
use math::tuple::{Point, Tuple, Vector};

const SIZE: usize = 500;
const WALL_Z: f32 = 10.0;
const WALL_SIZE: f32 = 7.0;
const PIXEL_SIZE: f32 = WALL_SIZE / (SIZE as f32);

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE, Color::black());
    let ray_origin = Point::new(0., 0., -5.);
    let s = Sphere::new();

    for y in 0..SIZE {
        let wall_y: f32 = WALL_SIZE / 2.0 - (y as f32) * PIXEL_SIZE;
        for x in 0..SIZE {
            let wall_x: f32 = WALL_SIZE / 2.0 - (x as f32) * PIXEL_SIZE;
            let wall = Point::new(wall_x, wall_y, WALL_Z);

            let r = Ray::new(
                ray_origin,
                Vector::try_from((Tuple::from(wall) - Tuple::from(ray_origin)).normalize())
                    .unwrap(),
            );
            let xs = s.intersect(&r);
            if let Some(_) = hit(&xs) {
                canvas.set(x, y, Color::white());
            }
        }
    }
    Image::from_canvas(canvas).write_png("circle.out.png");
}

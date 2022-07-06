use math::canvas::Canvas;
use math::color::Color;
use math::image::Image;
use math::intersect::{hit, Intersect};
use math::light::PointLight;
use math::lighting;
use math::material::Material;
use math::ray::Ray;
use math::sphere::Sphere;
use math::tuple::{Point, Tuple, Vector};
use math::world::WorldObject;

const SIZE: usize = 500;
const WALL_Z: f32 = 10.0;
const WALL_SIZE: f32 = 7.0;
const PIXEL_SIZE: f32 = WALL_SIZE / (SIZE as f32);

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE, Color::white());
    let ray_origin = Point::new(0., 0., -5.);

    let mut s = Sphere::new();
    s.set_material({
        let mut m = Material::new();
        m.color = Color::new(1.0, 0.2, 1.0);
        m
    });

    let light = PointLight {
        position: Tuple::point(-10., 10., -10.),
        intensity: Color::new(1., 1., 1.),
    };

    for y in 0..SIZE {
        let wall_y: f32 = WALL_SIZE / 2.0 - (y as f32) * PIXEL_SIZE;
        for x in 0..SIZE {
            let wall_x: f32 = (x as f32) * PIXEL_SIZE - WALL_SIZE / 2.0;
            let wall = Point::new(wall_x, wall_y, WALL_Z);

            let r = Ray::new(
                ray_origin,
                Vector::try_from((Tuple::from(wall) - Tuple::from(ray_origin)).normalize())
                    .unwrap(),
            );
            let xs = s.intersect(&r);
            if let Some(h) = hit(&xs) {
                let point = Tuple::from(r.position(h.t));
                let eye = -Tuple::from(r.direction);
                let color = lighting::phong(
                    h.object.material_at(point),
                    &light,
                    point,
                    eye,
                    h.object.normal_at(point),
                );
                canvas.set(x, y, color);
            }
        }
    }
    Image::from_canvas(canvas).write_png("sphere.out.png");
}

use math::canvas::Canvas;
use math::color::Color;
use math::image::Image;
use math::tuple::Tuple;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let dt = 0.01;
    let position = proj.position + proj.velocity * dt;
    let velocity = proj.velocity + env.gravity * dt + env.wind * dt;
    Projectile { position, velocity }
}

fn main() {
    println!("Hello");
    let mut canvas = Canvas::new(128, 128, Color::black());
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            canvas.set(
                x,
                y,
                Color::new(0.0, (y as f32) / 128.0, (x as f32) / 128.0),
            );
        }
    }

    let img = Image::from_canvas(canvas);
    img.write_png("img.out.png");
    img.write_ppm("img.out.ppm");

    let start = Tuple::point(0., 1., 0.);

    let gravity = Tuple::vector(0., -0.1, 0.);
    let wind = Tuple::vector(-0.01, 0., 0.);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 500, Color::black());
    for velocity_y in 2..5 {
        let velocity = Tuple::vector(2., velocity_y as f32, 0.).normalize() * 11.25;
        let mut color_i = [0.0, 0.0, 0.0];
        color_i[velocity_y - 2] = 1.0;
        let color = Color::new(color_i[0], color_i[1], color_i[2]);
        let mut p = Projectile {
            position: start,
            velocity: velocity,
        };
        while p.position.y > 0.0 && p.position.x < 900.0 {
            let x = (p.position.x.round() as usize).clamp(0, c.width - 1);
            let y = (500 - (p.position.y.round() as usize)).clamp(0, c.height - 1);
            c.set(x, y, color);
            p = tick(&e, &p);
        }
    }
    Image::from_canvas(c).write_png("proj.out.png");
}

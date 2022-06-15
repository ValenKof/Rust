use math::canvas::Canvas;
use math::color::Color;
use math::image::Image;

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
}

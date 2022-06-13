use math::image::Image;

fn main() {
    println!("Hello");
    let mut img = Image::new(128, 128);
    let mut next = 0;
    for row in 0..img.rows {
        for _col in 0..img.cols {
            img.data[next + 0] = 0;
            img.data[next + 1] = row as u8;
            img.data[next + 2] = 0;
            next += 3;
        }
    }

    img.write("img.png");
}

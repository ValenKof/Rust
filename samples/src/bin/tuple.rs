use math::tuple::{dot, Tuple};

fn angle(a: Tuple, b: Tuple) -> f32 {
    (dot(a, b) / (a.len() * b.len())).acos() * (180.0 / std::f32::consts::PI)
}

fn main() {
    println!("Point: {:?}", Tuple::point(1., 2., 3.));
    println!("Vector: {:?}", Tuple::vector(1., 2., 3.));

    let a = Tuple::vector(-1.0, 1.0, 0.0);
    let b = Tuple::vector(2.0, 2.0, 0.0);
    println!("Angle between {:?} and {:?}: {}", a, b, angle(a, b));
}

use math::tuple::Tuple;

fn main() {
    print!("Primes up to 100: ");
    for prime in samples::gen_primes(100) {
        print!("{} ", prime);
    }
    println!();

    print!("First 10 primes: ");
    for prime in samples::Primes::iter().take(10) {
        print!("{} ", prime);
    }
    println!();

    println!("Point: {:?}", Tuple::point(1., 2., 3.));
    println!("Vector: {:?}", Tuple::vector(1., 2., 3.));
}

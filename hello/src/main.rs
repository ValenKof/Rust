use math::tuple::Tuple;

fn main() {
    print!("Primes up to 100: ");
    for prime in primes::gen_primes(100) {
        print!("{} ", prime);
    }
    println!();

    println!("Point: {:?}", Tuple::point(1., 2., 3.));
    println!("Vector: {:?}", Tuple::vector(1., 2., 3.));
}

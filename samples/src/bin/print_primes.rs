use samples::primes;

fn main() {
    print!("Primes up to 100: ");
    for prime in primes::gen_primes(100) {
        print!("{} ", prime);
    }
    println!();

    print!("First 10 primes: ");
    for prime in primes::Primes::iter().take(10) {
        print!("{} ", prime);
    }
    println!();
}

fn main() {
    print!("Primes up to 100: ");
    for prime in primes::gen_primes(100) {
        print!("{} ", prime);
    }
    println!();
}

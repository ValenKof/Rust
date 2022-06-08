fn main() {
    println!("Primes around ...");
    let mut base: u64 = 1;
    for power in 1..=15 {
        base *= 10;

        let mut below = base - 1;
        while !primes::is_prime(below) {
            below -= 2;
        }

        let mut above = base + 1;
        while !primes::is_prime(above) {
            above += 2;
        }

        println!("10 ** {:2}: {} {}", power, below, above);
    }
}

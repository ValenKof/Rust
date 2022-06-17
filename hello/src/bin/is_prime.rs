use std::str::FromStr;

fn main() {
    for arg in std::env::args().skip(1) {
        if let Ok(p) = u64::from_str(&arg) {
            println!(
                "Checking {}: {}PRIME",
                p,
                if primes::is_prime(p) { "" } else { "NOT " }
            );
        } else {
            println!("Skipping '{}'", arg);
        }
    }
}

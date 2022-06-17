# Rust Samples

Code samples in Rust.

## Content

* `math` - graphics library (and example binaries)
* `primes` - primes library (and example binaries)
* `hello` - binaries using `math` and `primes` libraries

## Usage

```
cd ~ && git clone https://github.com/ValenKof/Rust.git
cd ~/Rust/math && cargo build && cargo run
cd ~/Rust/primes && cargo build && cargo run
cd ~/Rust/hello
cargo run --bin print_primes
cargo run --bin is_prime 1 two 3 4.0 5 6
cargo run --bin stack
```

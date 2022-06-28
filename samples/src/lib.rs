pub fn is_prime(n: u64) -> bool {
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        return n == 2 || n == 3 || n == 5;
    }
    let mut d = 7;
    let mut c = 4;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += c;
        c ^= 4 ^ 2;
    }
    n > 1
}

pub fn gen_primes(max_num: u64) -> Vec<u64> {
    let mut primes = vec![];
    for num in 0..=max_num {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

pub struct Primes {
    next: u64,
}

impl Primes {
    pub fn iter() -> Primes {
        Primes { next: 1 }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.next) {
            self.next += 1;
        }
        let res = self.next;
        self.next += 1;
        Some(res)
    }
}

#[test]
fn test_gen_primes() {
    assert_eq!(gen_primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert_eq!(
        Primes::iter().take(6).collect::<Vec<u64>>(),
        vec![2, 3, 5, 7, 11, 13]
    );
    assert_eq!(gen_primes(1_000_000).len(), 78498);
}

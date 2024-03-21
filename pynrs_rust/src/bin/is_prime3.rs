use std::sync::OnceLock;
use pynrs_rust::isqrt::isqrt;
use pynrs_rust::Eratosthenes;


fn calulate_known_primes() -> Vec<u32> {
    let max_n = 2_u64.pow(32);
    eprintln!("creating sieve of size {max_n}. patience! this might need some minutes.");
    let t = std::time::Instant::now();
    let erato = Eratosthenes::create(max_n);
    let dt = t.elapsed();
    println!("creating sieve of size {max_n} in Rust needed {} s", dt.as_secs());

    eprintln!("converting sieve into list of primes. more patience!");
    let t = std::time::Instant::now();
    let mut known_primes = Vec::new();
    for prime in erato.prime_iter(max_n) {
        let prime: u32 = prime.try_into().unwrap();
        known_primes.push(prime);
    }
    let dt = t.elapsed();
    println!("converting sieve into list of primes with Rust needed {} s", dt.as_secs());
    known_primes
}

fn get_known_primes() -> &'static Vec<u32> {
    static KNOWN_PRIMES: OnceLock<Vec<u32>> = OnceLock::new();
    KNOWN_PRIMES.get_or_init(|| calulate_known_primes())
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let max_i = isqrt(n);
    for i in get_known_primes() {
        let i = *i as u64;
        if i > max_i {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for i in [13, 17, 19, 31] {
        assert!(is_prime(2_u64.pow(i) - 1));
    }
    for i in [15, 21, 27, 29] {
        assert!(!is_prime(2_u64.pow(i) - 1));
    }
    let t = std::time::Instant::now();
    assert!(is_prime(2_u64.pow(61) - 1));
    println!("Mersenne Prime 61 in Rust with list of ints needed {:.3} s",
        t.elapsed().as_secs_f32());

    let t = std::time::Instant::now();
    assert!(is_prime(u64::MAX - 58));
    println!("proving that 2**64-59 is prime in Rust with list of ints needed {:.3} s",
        t.elapsed().as_secs_f32());
}

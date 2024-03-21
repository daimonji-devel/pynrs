use std::sync::OnceLock;
use pynrs_rust::isqrt::isqrt;
use pynrs_rust::Eratosthenes;


fn calulate_eratosthenes() -> Eratosthenes {
    let max_n = 2_u64.pow(32);
    eprintln!("creating sieve of size {max_n}. patience! this might need some minutes.");
    let t = std::time::Instant::now();
    let erato = Eratosthenes::create(max_n);
    let dt = t.elapsed();
    println!("creating sieve of size {max_n} in Rust needed {} s", dt.as_secs());
    erato
}

fn get_eratosthenes() -> &'static Eratosthenes {
    static ERATOSTHENES: OnceLock<Eratosthenes> = OnceLock::new();
    ERATOSTHENES.get_or_init(|| calulate_eratosthenes())
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in get_eratosthenes().prime_iter(isqrt(n)) {
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
    println!("Mersenne Prime 61 in Rust with boolarray needed {:.3} s",
        t.elapsed().as_secs_f32());

    let t = std::time::Instant::now();
    assert!(is_prime(u64::MAX - 58));
    println!("proving that 2**64-59 is prime in Rust with boolarray needed {:.3} s",
        t.elapsed().as_secs_f32());
}

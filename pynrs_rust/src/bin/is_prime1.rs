use pynrs_rust::isqrt::isqrt;


fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let limit = isqrt(n) + 1;
    for i in 2..limit {
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
    println!("Mersenne Prime 61 in Rust needed {:.3} s",
        t.elapsed().as_secs_f32());

    let t = std::time::Instant::now();
    assert!(is_prime(u64::MAX - 58));
    println!("proving that 2**64-59 is prime in Rust needed {:.3} s",
        t.elapsed().as_secs_f32());
}

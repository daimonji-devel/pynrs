use pyo3::prelude::*;
use std::sync::OnceLock;

use pynrs_rust::Eratosthenes;
use pynrs_rust::isqrt::isqrt;

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

/// checks whether a u64 number is prime.
/// compiles a list of prime numbers until 2**32 on first call.
#[pyfunction]
fn is_prime(n: u64) -> PyResult<bool> {
    if n < 2 {
        return Ok(false);
    }
    let max_i = isqrt(n);
    for i in get_known_primes() {
        let i = *i as u64;
        if i > max_i {
            break;
        }
        if n % i == 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

#[pymodule]
fn pynrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    Ok(())
}

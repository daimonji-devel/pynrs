use crate::isqrt::isqrt;
use crate::BoolArray;
use crate::BoolArrayIterator;


#[derive(Debug)]
pub struct Eratosthenes {
    sieve: BoolArray,
}

impl Eratosthenes {

    pub fn create(n_max: u64) -> Self {
        let k_max = isqrt(n_max);
        let mut sieve = BoolArray::new(true, n_max + 1);
        sieve.set(0, false);
        if n_max > 0 {
            sieve.set(1, false);
        }
        for k in 2 .. k_max + 1 {
            if sieve.get(k) {
                let mut l = k * k;
                let l_max = n_max - n_max % k;
                loop {
                    sieve.set(l, false);
                    if l >= l_max {
                        break
                    }
                    l += k;
                }
            }
        }
        Eratosthenes {
            sieve: sieve,
        }
    }

    pub fn is_prime(self: &Self, n: u64) -> bool {
        self.sieve.get(n)
    }

    pub fn prime_iter(&self, max_n: u64) -> PrimeIterator {
        if max_n >= self.sieve.len() {
            panic!("sieve not large enough for max_n {max_n}")
        }
        PrimeIterator::create(&self.sieve, max_n)
    }

    pub fn count_primes(self: &Self, n_max: u64) -> u64 {
        let mut prime_count = 0;
        if n_max > 0 {
            let mut number_count = 0;
            for value in self.sieve.iter() {
                if value {
                    prime_count += 1;
                }
                number_count += 1;
                if number_count > n_max {
                    break;
                }
            }
        }
        prime_count
    }

}

pub struct PrimeIterator<'a> {
    is_primes: BoolArrayIterator<'a>,
    n: u64,
    max_n: u64,
}

/// Iterator over a sieve of Eratosthenes extracting all primes until a max_value.
impl PrimeIterator<'_> {
    fn create(sieve: &BoolArray, max_n: u64) -> PrimeIterator {
        PrimeIterator {
            is_primes: sieve.iter(),
            n: 0,
            max_n: max_n,
        }
    }
}

impl Iterator for PrimeIterator<'_> {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        while self.n <= self.max_n {
            match self.is_primes.next() {
                None => {
                    return None;
                },
                Some(is_prime) => {
                    let n = self.n;
                    self.n += 1;
                    if is_prime {
                        return Some(n);
                    }
                },
            }
        }
        None
    }
}

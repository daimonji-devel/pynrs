import sys
import time
from isqrt import isqrt
from Eratosthenes import Eratosthenes

max_n = isqrt(2 ** 61 - 1)
t = time.time()
print(f"creating sieve of size {max_n}. patience! this needs some 15 minutes.", file=sys.stderr)
known_primes = Eratosthenes.create(max_n)
dt = round(time.time() - t)
print(f'creating sieve of size {max_n} with Python needed {dt} s')

def is_prime(n):
    if n < 2:
        return False
    max_i = isqrt(n)
    for i in known_primes:
        if i > max_i:
            break
        if n % i == 0:
            return False
    return True

if __name__ == '__main__':
    for e in (13, 17, 19, 31):
        assert is_prime(2 ** e - 1)

    for e in (15, 21, 27, 29):
        assert not is_prime(2 ** e - 1)

    print('patience. calculation needs some minutes...', file=sys.stderr)
    t = time.time()
    assert is_prime(2 ** 61 - 1)
    print(f'Mersenne Prime 61 in Python with boolarray needed {time.time() - t:.3f} s')

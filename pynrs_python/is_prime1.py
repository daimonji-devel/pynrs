import sys
import time
from isqrt import isqrt


def is_prime(n):
    if n < 2:
        return False
    max_i = isqrt(n)
    for i in range(2, max_i + 1):
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
    print(f'Mersenne Prime 61 in Python needed {time.time() - t:.3f} s')

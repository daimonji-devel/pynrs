from BoolArray import BoolArray
from isqrt import isqrt


class Eratosthenes:

    def __init__(self, sieve):
        """
        Do not call init but the class function create().
        """
        self._sieve = sieve

    @classmethod
    def create(cls, n_max: int):
        k_max = isqrt(n_max)
        sieve = BoolArray(init_len=n_max + 1, init_value=True)
        sieve[0] = False
        sieve[1] = False
        for k in range(2, k_max + 1):
            if sieve[k]:
                l_min = k * k
                l_max = n_max - n_max % k
                for l in range(l_min, l_max + 1, k):
                    sieve[l] = False
        return cls(sieve)

    def __getitem__(self, n: int) -> bool:
        if n < 2:
            # 0 and 1 would be covered in sieve. But negative numbers need to be caught because there are
            # legal negative list indices
            return False
        return self._sieve[n]

    def __iter__(self):
        for number, is_prime in enumerate(self._sieve):
            if is_prime:
                yield number

    def n_max(self):
        return len(self._sieve) - 1


if __name__ == '__main__':
    primes = Eratosthenes.create(2**22)

    assert not primes[0]
    assert not primes[1]
    assert primes[2]
    assert primes[3]
    assert not primes[4]
    assert primes[5]

    for e in (13, 17, 19):
        assert primes[2 ** e - 1]

    for e in (11, 15, 21):
        assert not primes[2 ** e - 1]

    assert sum(1 for _ in primes) == 295947

    import time
    e = 28
    print(f"creating sieve of size 2**{e}. patience! this needs some minutes.")
    t = time.time()
    primes = Eratosthenes.create(2**e)
    dt = round(time.time() - t)
    print(f'creating sieve of size 2**{e} with Python needed {dt} s')

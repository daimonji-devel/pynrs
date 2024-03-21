import time
from pynrs import is_prime


if __name__ == '__main__':
    for e in (13, 17, 19, 31):
        assert is_prime(2 ** e - 1) == True

    for e in (15, 21, 27, 29):
        assert is_prime(2 ** e - 1) == False

    t = time.time()
    assert is_prime(2 ** 61 - 1) == True
    dt = time.time() - t
    print(f'proving Mersenne Prime 61 with Rust from Python needed {dt:5.3f} s')

    t = time.time()
    assert is_prime(2 ** 64 - 59) == True
    dt = time.time() - t
    print(f'proving that 2**64-59 with Rust from Python needed {dt:5.3f} s')

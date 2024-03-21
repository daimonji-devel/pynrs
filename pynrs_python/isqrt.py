def isqrt(n: int) -> int:
    """
    Calculates the integer square root.
    The result is the rounded down version: result ** 2 <= self < (result + 1) ** 2.
    @param n number to take square root from
    @return the integer square root.
    @raises ValueError if n < 0.
    """
    shift = 2
    while (n >> shift) > 0:
        shift += 2

    result = 0
    while shift >= 0:
        result <<= 1
        result += 1
        if result * result > n >> shift:
            result -= 1
        shift -= 2
    return result


if __name__ == '__main__':
    assert isqrt(0) == 0
    assert isqrt(1) == 1
    assert isqrt(2) == 1
    assert isqrt(3) == 1
    assert isqrt(4) == 2
    for n in [2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 1000, 2**8, 2**16, 2**64]:
        assert isqrt(n ** 2 - 1) == n - 1
        assert isqrt(n ** 2) == n
        assert isqrt(n ** 2 + 1) == n
        assert isqrt((n + 1) ** 2 - 1) == n
        assert isqrt((n + 1) ** 2) == n + 1

pub fn isqrt(n: u64) -> u64 {
    // inspired by wikipedia article about integer square roots
    let mut shift = 2;
    while (n.checked_shr(shift).unwrap_or(0)) > 0 {
        shift += 2;
    }

    let mut result = 0;
    loop {
        result <<= 1;
        let large_cand = result + 1;
        if large_cand * large_cand <= n.checked_shr(shift).unwrap_or(0) {
            result = large_cand;
        }
        if shift == 0 {
            break;
        }
        shift -= 2;
    }
    result
}

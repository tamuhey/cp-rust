pub fn comb(m: usize, n: usize) -> usize {
    if m < n {
        return 0;
    }
    let mut ret = 1;
    for i in (m - n)..m {
        ret *= i + 1;
    }
    for i in 0..n {
        ret /= i + 1
    }
    ret
}

pub fn combmodlucas(m: usize, n: usize, p: usize) -> usize {
    let mut m = m;
    let mut n = n;
    if m < n {
        return 0;
    }
    let mut ret = 1;
    loop {
        ret *= comb(m % p, n % p);
        if m == 0 {
            break;
        }
        m /= p;
        n /= p;
    }
    ret
}

// 大きめのcombinationについて桁溢れしないように求める
pub fn comb_large(a: u128, b: u128) -> u128 {
    let mut ret = 1u128;
    let mut i = a - b + 1;
    let mut j = 2;
    while i <= a || j <= b {
        ret *= i;
        i += 1;
        while ret % j == 0 && j <= b {
            ret /= j;
            j += 1;
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lucas() {
        let cases = &[(3, 13, 3, 1)];
        for &(p, m, n, expected) in cases {
            assert_eq!(expected, combmodlucas(m, n, p))
        }
    }
    #[test]
    fn test_comb() {
        assert_eq!(comb(18, 9), 48620);
    }

    #[test]
    fn test_comb_large() {
        assert_eq!(comb_large(18, 9), 48620);
    }
}

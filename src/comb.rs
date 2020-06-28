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
}

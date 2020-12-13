pub fn comb(m: u128, n: u128) -> u128 {
    if m < n {
        0
    } else if n == 0 {
        1
    } else {
        comb(m - 1, n - 1) * m / n
    }
}

pub fn combmodlucas(m: usize, n: usize, p: usize) -> usize {
    let mut m = m;
    let mut n = n;
    if m < n {
        return 0;
    }
    let mut ret = 1;
    loop {
        ret *= comb((m % p) as u128, (n % p) as u128);
        if m == 0 {
            break;
        }
        m /= p;
        n /= p;
    }
    ret as usize
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
        assert_eq!(comb(50, 25), 126410606437752);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lca(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

/// 拡張ユークリッド互除法
/// ax + by = c
/// verified: https://atcoder.jp/contests/acl1/submissions/16916747
/// returns: (gcd == c, x, y)
pub fn ext_gcd(a: u128, b: u128) -> (u128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, y, x) = ext_gcd(b, a % b);
    // (c, x, y)
    (d, x, y - ((a / b) as i128) * x)
}

use std::collections::HashMap;
pub fn factor(mut n: usize) -> HashMap<usize, usize> {
    let mut ret = std::collections::HashMap::new();
    let n0 = n;
    let mut cur = 2;
    while cur * cur <= n0 {
        if n % cur != 0 {
            cur += 1;
            continue;
        }
        let mut count = 0;
        while n % cur == 0 {
            n /= cur;
            count += 1;
        }
        ret.insert(cur, count);
        cur += 1;
    }
    if n > 1 {
        ret.insert(n, 1);
    }
    ret
}

pub fn num_divs(facts: &HashMap<usize, usize>) -> usize {
    facts.iter().map(|(_, v)| v + 1).product()
}

pub fn msb(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 1 {
        n >>= 1;
        ret += 1;
    }
    ret
}

pub fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}

use ac_library_rs::modint::ModInt1000000007 as M;
pub fn pow_mod(x: usize, mut a: usize) -> M {
    let mut cur = M::new(x);
    let mut ret = M::new(1);
    while a > 0 {
        if a & 1 == 1 {
            ret *= cur;
        }
        a >>= 1;
        cur *= cur;
    }
    ret
}

// baby step giant step
pub fn baby_step_giant_step(g: usize, h: usize) -> Option<u32> {
    let n = M::modulus() + 10;
    let m = (n as f64).sqrt().ceil() as u32 + 100;
    assert!((m as usize).pow(2) >= n as usize);
    let table = {
        let mut table = HashMap::new();
        let mut cur = M::new(1);
        for i in 0..m {
            table.insert(cur, i);
            cur *= g;
        }
        table
    };

    let mut y = M::new(h);
    let factor = pow_mod(g, (M::modulus() - m - 1) as usize);
    for i in 0..=m {
        if let Some(j) = table.get(&y) {
            return Some(i * m + j);
        }
        y *= factor;
    }
    None
}

// 一般化pow
use num::One;
use std::ops::Mul;
fn pow<T>(x: T, n: usize) -> T::Output
where
    T: One + Mul + Copy,
{
    let mut n = n;
    let mut cur: T = x;
    let mut ret: T = One::one();
    while n > 0 {
        if n & 1 != 0 {
            ret = ret * cur;
        }
        cur = cur * cur;
        n >>= 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_divs() {
        assert_eq!(num_divs(&factor(1)), 1);
        assert_eq!(num_divs(&factor(2)), 2);
        assert_eq!(num_divs(&factor(4)), 3);
        assert_eq!(num_divs(&factor(8)), 4);
        assert_eq!(num_divs(&factor(6)), 4);
        assert_eq!(num_divs(&factor(57)), 4);
        assert_eq!(num_divs(&factor(60)), 12);
    }

    #[quickcheck]
    fn test_pow(x: f64, y: i32) -> bool {
        if x.is_nan() || y < 0 {
            true
        } else {
            x.powi(y) == pow(x, y as usize)
        }
    }
}

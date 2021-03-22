fn get_prime_table(n: usize) -> Vec<bool> {
    let mut ret = vec![true; n];
    ret[0] = false;
    ret[1] = false;
    for i in 2..n {
        if !ret[i] {
            continue;
        }
        for j in (i + i..n).step_by(i) {
            ret[j] = false
        }
    }
    ret
}

fn get_primes(lim: usize) -> Vec<usize> {
    let tb = get_prime_table(lim + 1);
    (2..=lim).filter(|&i| tb[i]).collect()
}

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

use ac_library_rs::math::inv_mod;
/// ax = b (mod m)
/// returns (x, m)
pub fn linear_congruence_one(a: i64, b: i64, m: i64) -> Option<(i64, i64)> {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    let d = gcd(a, m);
    if b % d != 0 {
        return None;
    }
    let md = m / d;
    let x = (b / d) % md;
    let y = inv_mod(a / d, md) % md;
    Some(((x * y) % md, md))
}

#[test]
fn test_linear_congruence_one() {
    assert_eq!(linear_congruence_one(3, 4, 5), Some((3, 5)));
    assert_eq!(linear_congruence_one(2, 4, 6), Some((2, 3)));
    assert_eq!(linear_congruence_one(3, 4, 6), None);
}
pub fn linear_congruence(av: &[i64], bv: &[i64], mv: &[i64]) -> Option<(i64, i64)> {
    let mut x = 0;
    let mut m = 1;
    for i in 0..av.len() {
        let a = av[i] * m;
        let b = bv[i] - av[i] * x;
        if let Some((t, md)) = linear_congruence_one(a, b, mv[i]) {
            x += m * t;
            m *= md;
            x %= m;
        } else {
            return None;
        }
    }
    x %= m;
    if x < 0 {
        x += m
    };
    Some((x, m))
}

#[cfg(test)]
#[quickcheck]
fn test_linear_congruence(a: u32, b: u32, m: u32) {
    let a = (a / 3) as i64;
    let b = (b / 3) as i64;
    let m = (m / 3) as i64;
    if m == 0 || a == 0 {
        return;
    }
    let x = linear_congruence_one(a, b, m);
    let y = linear_congruence(&[a], &[b], &[m]);
    assert_eq!(x, y, "case: {:?}", (a, b, m))
}

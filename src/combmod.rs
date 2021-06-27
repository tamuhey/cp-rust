use ac_library_rs::{Mod1000000007, Modulus, StaticModInt};
use lazy_static::lazy_static;
lazy_static! {
    static ref comb: CombMod::<Mod1000000007> = CombMod::new(1_000_000);
}

pub fn get_factorials<M: Modulus>(n: usize) -> Vec<StaticModInt<M>> {
    let mut facts = vec![0.into(); n];
    facts[0] = 1.into();
    for i in 1..n {
        facts[i] = facts[i - 1] * StaticModInt::<M>::new(i);
    }
    facts
}

pub fn get_facinvs<M: Modulus>(n: usize, facts: &Vec<StaticModInt<M>>) -> Vec<StaticModInt<M>> {
    let mut invs = vec![0.into(); n];
    invs[n - 1] = facts[n - 1].inv();
    for i in (0..(n - 1)).rev() {
        invs[i] = invs[i + 1] * StaticModInt::new(i + 1)
    }
    invs
}

pub fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn prime_factors(x: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut y = x;
    let mut p = 2;
    let upper = (y as f64).sqrt() as usize;
    while y > 1 && p <= upper {
        match y % p {
            0 => {
                res.push(p);
                y /= p;
            }
            _ => {
                p += 1;
            }
        }
    }
    if y > 1 {
        res.push(y)
    }
    return res;
}

pub struct CombMod<M: Modulus> {
    facts: Vec<StaticModInt<M>>,
    invs: Vec<StaticModInt<M>>,
}

impl<M: Modulus> CombMod<M> {
    pub fn new(n: usize) -> Self {
        let facts = get_factorials(n);
        let invs = get_facinvs(n, &facts);
        CombMod { facts, invs }
    }
    pub fn c(&self, a: usize, b: usize) -> StaticModInt<M> {
        if a < b {
            0.into()
        } else {
            (self.facts[a] * self.invs[b]) * self.invs[a - b]
        }
    }
    pub fn p(&self, a: usize, b: usize) -> StaticModInt<M> {
        if b > a {
            0.into()
        } else {
            self.facts[a] * self.invs[a - b]
        }
    }
    pub fn h(&self, a: usize, b: usize) -> StaticModInt<M> {
        if a + b == 0 {
            0.into()
        } else {
            self.c(a + b - 1, b)
        }
    }
}

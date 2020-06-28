#![allow(dead_code)]

use crate::mint::*;

pub fn get_factorials(n: usize) -> Vec<Mint<usize>> {
    let mut facts = vec![Mint(0); n];
    facts[0] = Mint(1);
    for i in 1..n {
        facts[i] = facts[i - 1] * (i as usize);
    }
    facts
}

pub fn get_facinvs(n: usize, facts: &Vec<Mint<usize>>) -> Vec<Mint<usize>> {
    let mut invs = vec![Mint(0); n];
    invs[n - 1] = facts[n - 1].inv();
    for i in (0..(n - 1)).rev() {
        invs[i] = invs[i + 1] * (i + 1)
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

pub struct CombMod {
    facts: Vec<Mint<usize>>,
    invs: Vec<Mint<usize>>,
}

impl CombMod {
    pub fn new(n: usize) -> Self {
        let facts = get_factorials(n);
        let invs = get_facinvs(n, &facts);
        CombMod {
            facts: facts,
            invs: invs,
        }
    }
    pub fn c(&self, a: usize, b: usize) -> Mint<usize> {
        if a < b {
            Mint(0)
        } else {
            (self.facts[a] * self.invs[b]) * self.invs[a - b]
        }
    }
    pub fn p(&self, a: usize, b: usize) -> Mint<usize> {
        if b > a {
            Mint(0)
        } else {
            self.facts[a] * self.invs[a - b]
        }
    }
}

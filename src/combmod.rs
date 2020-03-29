#![allow(dead_code)]

pub mod combmod {
    use super::mint::mint::*;

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

    fn gcd(a: usize, b: usize) -> usize {
        match b {
            0 => a,
            _ => gcd(b, a % b),
        }
    }

    fn prime_factors(x: usize) -> Vec<usize> {
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

    struct CombMod {
        facts: Vec<Mint<usize>>,
        invs: Vec<Mint<usize>>,
    }

    impl CombMod {
        fn new(n: usize) -> Self {
            let facts = get_factorials(n);
            let invs = get_facinvs(n, &facts);
            CombMod {
                facts: facts,
                invs: invs,
            }
        }
        fn get(&self, a: usize, b: usize) -> Mint<usize> {
            if a < b {
                Mint(0)
            } else {
                (self.facts[a as usize] * self.invs[b as usize]) * self.invs[(a - b) as usize]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(8, 44), 4);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(10), vec![2, 5]);
        assert_eq!(prime_factors(12387), vec![3, 4129]);
        assert_eq!(prime_factors(16), vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_combmod() {
        let combmod = CombMod::new(100000);
        assert_eq!(combmod.get(100, 50).0, 538992043);
        assert_eq!(combmod.get(10000, 3001).0, 981292794);
        assert_eq!(combmod.get(10000, 30001).0, 0);
    }
}

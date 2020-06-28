const M: usize = 1 << 31;
const A: usize = 22695477;
const C: usize = 1;
pub struct Rng {
    pub state: usize,
}

impl Rng {
    pub fn new(state: usize) -> Self {
        Rng { state: state }
    }
}

impl Iterator for Rng {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let state = self.state;
        self.state = (self.state * A + C) % M;
        Some(state)
    }
}

pub struct Rand {
    pub rng: Rng,
}

impl Rand {
    pub fn new(state: usize) -> Self {
        Rand {
            rng: Rng::new(state),
        }
    }
    pub fn uniform(&mut self, low: f64, high: f64) -> f64 {
        let x = self.rng.next().unwrap() as f64;
        x * (high - low) / (M as f64) + low
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_uniform() {
        let mut rnd = Rand::new(0);
        let low = -10.;
        let high = 100.;
        let ntry = 1000000;
        let eps = 0.1;
        let rands = (0..ntry)
            .map(|_| rnd.uniform(low, high))
            .collect::<Vec<_>>();
        assert!(rands.iter().all(|&x| x >= low && x <= high));
        assert!(rands.iter().any(|&x| x < low + eps || x > high - eps));
    }
}

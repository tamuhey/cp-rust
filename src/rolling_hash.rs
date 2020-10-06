// Verified: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4891701#1

struct RollingHash {
    hashes: Vec<usize>,
    bases: Vec<usize>,
}
impl RollingHash {
    const base: usize = 1_000_000_007;
    fn new(s: &str) -> Self {
        let mut h = 0usize;
        let mut b = 1usize;
        let mut hashes = vec![0];
        let mut bases = vec![1];
        for c in s.bytes() {
            h = Self::add(h, c);
            hashes.push(h);
            b = b.wrapping_mul(Self::base);
            bases.push(b);
        }
        Self { hashes, bases }
    }

    fn find_all(&self, target: &str) -> Vec<usize> {
        let mut th = 0usize;
        let mut nt = 0usize;
        let n = self.hashes.len() - 1;
        for c in target.bytes() {
            th = Self::add(th, c);
            nt += 1;
            if nt > n {
                return vec![];
            }
        }
        let mut ret = vec![];
        for l in 0..=(n - nt) {
            if th == self.get_hash(l, l + nt) {
                ret.push(l)
            }
        }
        ret
    }
    fn get_hash(&self, l: usize, r: usize) -> usize {
        self.hashes[r].wrapping_sub(self.hashes[l].wrapping_mul(self.bases[r - l]))
    }
    fn add(hash: usize, byte: u8) -> usize {
        hash.wrapping_mul(Self::base).wrapping_add(byte as usize)
    }
}
#[cfg(test)]
mod test {
    use super::RollingHash;
    #[test]
    fn find_all() {
        for (a, b, expected) in &[
            ("aafasdfasfdaafgga", "aa", vec![0, 11]),
            ("wowowow", "wo", vec![0, 2, 4]),
        ] {
            let rh = RollingHash::new(a);
            assert_eq!(&rh.find_all(b), expected);
        }
    }
}

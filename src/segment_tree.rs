// verified in https://atcoder.jp/contests/arc008/submissions/10187379
use crate::monoid::*;
pub struct SegTree<T: Monoid + Clone> {
    dat: Vec<T>,
    n: usize,
}

impl<T: Monoid + Copy> SegTree<T> {
    pub fn new(n: usize) -> Self {
        let dat = vec![T::unit(); n << 1];
        SegTree { dat, n }
    }
    pub fn new_with(a: impl ExactSizeIterator<Item = T>) -> Self {
        let n = a.len();
        let mut ret = Self::new(n);
        for (i, x) in a.enumerate() {
            ret.update(i, x)
        }
        ret
    }
    pub fn update(&mut self, k: usize, v: T) {
        let mut k = k + self.n;
        self.dat[k] = v;
        while {
            k >>= 1;
            k > 0
        } {
            let pk = k << 1;
            self.dat[k] = T::add(&self.dat[pk], &self.dat[pk | 1]);
        }
    }
    pub fn get(&self, a: usize, b: usize) -> T {
        let mut a = a + self.n;
        let mut b = b + self.n;
        let mut va = T::unit();
        let mut vb = T::unit();
        while a < b {
            if a & 1 != 0 {
                va = T::add(&va, &self.dat[a]);
                a += 1;
            }
            if b & 1 != 0 {
                vb = T::add(&self.dat[b - 1], &vb);
            }
            a >>= 1;
            b >>= 1;
        }
        T::add(&va, &vb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monoid::Min;
    #[quickcheck]
    fn test_min(mut a: Vec<usize>, b: Vec<(usize, usize)>) {
        let n = a.len();
        let mut sg = SegTree::new_with(a.iter().cloned().map(Min));
        eprintln!("{:?}", sg.dat); // DEBUG
        for &(i, bi) in &b {
            if i < n {
                a[i] = bi;
                sg.update(i, Min(bi));
                assert_eq!(*a.iter().min().unwrap(), sg.get(0, n).0)
            }
        }
    }
}

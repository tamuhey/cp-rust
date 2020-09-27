// verified in https://atcoder.jp/contests/arc008/submissions/10187379
use crate::monoid::*;
pub struct SegTree<T>
where
    T: Monoid,
{
    dat: Vec<T::S>,
    n: usize,
}

use std::borrow::Borrow;
impl<T: Monoid, I> From<I> for SegTree<T>
where
    T::S: Copy,
    I: IntoIterator,
    I::Item: Borrow<T::S>,
    I::IntoIter: ExactSizeIterator,
{
    fn from(a: I) -> Self {
        let a = a.into_iter();
        let n = a.len();
        let mut ret = Self::new(n);
        for (i, x) in a.enumerate() {
            ret.update(i, x.borrow().clone())
        }
        ret
    }
}

impl<T> SegTree<T>
where
    T: Monoid,
    T::S: Copy,
{
    pub fn new(n: usize) -> Self {
        let dat = vec![T::id(); n << 1];
        SegTree { dat, n }
    }
    pub fn update(&mut self, k: usize, v: T::S) {
        let mut k = k + self.n;
        self.dat[k] = v;
        while {
            k >>= 1;
            k > 0
        } {
            let pk = k << 1;
            self.dat[k] = T::op(&self.dat[pk], &self.dat[pk | 1]);
        }
    }
    pub fn get(&self, a: usize, b: usize) -> T::S {
        let mut a = a + self.n;
        let mut b = b + self.n;
        let mut va = T::id();
        let mut vb = T::id();
        while a < b {
            if a & 1 != 0 {
                va = T::op(&va, &self.dat[a]);
                a += 1;
            }
            if b & 1 != 0 {
                vb = T::op(&self.dat[b - 1], &vb);
            }
            a >>= 1;
            b >>= 1;
        }
        T::op(&va, &vb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monoid::Min;
    #[quickcheck]
    fn test_min(mut a: Vec<usize>, b: Vec<(usize, usize)>) {
        let n = a.len();
        let mut sg: SegTree<Min<_>> = (&a).into();
        for &(i, bi) in &b {
            if i < n {
                a[i] = bi;
                sg.update(i, bi);
                assert_eq!(*a.iter().min().unwrap(), sg.get(0, n))
            }
        }
    }
}

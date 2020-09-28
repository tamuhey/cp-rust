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
            ret.set(i, x.borrow().clone())
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
    pub fn set(&mut self, k: usize, v: T::S) {
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
    pub fn get(&self, mut l: usize, mut r: usize) -> T::S {
        l += self.n;
        r += self.n;
        let mut va = T::id();
        let mut vb = T::id();
        while l < r {
            if l & 1 != 0 {
                va = T::op(&va, &self.dat[l]);
                l += 1;
            }
            if r & 1 != 0 {
                vb = T::op(&self.dat[r - 1], &vb);
            }
            l >>= 1;
            r >>= 1;
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
                sg.set(i, bi);
                assert_eq!(*a.iter().min().unwrap(), sg.get(0, n))
            }
        }
    }
}

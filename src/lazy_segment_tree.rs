// use crate::monoid::*;

// pub trait MFunctor: Monoid {
//     type M: Monoid;
//     fn map(&self, m: Self::M) -> Self::M;
// }

// pub struct LazySegTree<T: Monoid + Clone> {
//     dat: Vec<T>,
//     lazy: Vec<Option<T>>,
//     n: usize,
// }

// impl<T: Monoid + Copy> LazySegTree<T> {
//     pub fn new(n: usize) -> Self {
//         let dat = vec![T::id(); n << 1];
//         let lazy = vec![None; n];
//         LazySegTree { dat, n, lazy }
//     }
//     pub fn new_with(a: impl ExactSizeIterator<Item = T>) -> Self {
//         let n = a.len();
//         let mut ret = Self::new(n);
//         for (i, x) in a.enumerate() {
//             ret.update(i, x)
//         }
//         ret
//     }
//     pub fn update(&mut self, k: usize, v: T) {
//         let mut k = k + self.n;
//         self.dat[k] = v;
//         while {
//             k >>= 1;
//             k > 0
//         } {
//             let pk = k << 1;
//             self.dat[k] = T::op(&self.dat[pk], &self.dat[pk | 1]);
//         }
//     }
//     pub fn get(&self, a: usize, b: usize) -> T {
//         let mut a = a + self.n;
//         let mut b = b + self.n;
//         let mut va = T::id();
//         let mut vb = T::id();
//         while a < b {
//             if a & 1 != 0 {
//                 va = T::op(&va, &self.dat[a]);
//                 a += 1;
//             }
//             if b & 1 != 0 {
//                 vb = T::op(&self.dat[b - 1], &vb);
//             }
//             a >>= 1;
//             b >>= 1;
//         }
//         T::op(&va, &vb)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::monoid::Min;
//     #[test]
//     fn mfunctor() {
//         struct Change<T>(T);
//         // impl<T> MFunctor for Change<T> {}
//     }
//     #[quickcheck]
//     fn test_min(mut a: Vec<usize>, b: Vec<(usize, usize)>) {}
// }

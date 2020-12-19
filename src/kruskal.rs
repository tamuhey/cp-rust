use ac_library_rs::dsu::Dsu;
use num::Zero;
use std::cmp::Reverse;
use std::cmp::{Eq, Ord, Ordering};
use std::collections::BinaryHeap;
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct S<T>(T);
impl<T: PartialEq> Eq for S<T> {}
impl<T> Ord for S<T>
where
    T: PartialOrd + PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// verified https://atcoder.jp/contests/arc029/submissions/18857825
pub fn kruskal<T>(n: usize, es: &[(usize, usize, T)]) -> Vec<(usize, usize, T)>
where
    T: PartialOrd + PartialEq + Zero + Clone + Copy,
{
    let mut ret = vec![];
    let mut q: BinaryHeap<_> = es
        .iter()
        .cloned()
        .map(|(x, y, z)| (Reverse(S(z)), x, y))
        .collect();
    let mut uf = Dsu::new(n);
    while let Some((Reverse(S(c)), i, j)) = q.pop() {
        if !uf.same(i, j) {
            uf.merge(i, j);
            ret.push((i, j, c));
        }
    }
    ret
}

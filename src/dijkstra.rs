use num::Zero;
use std::cmp::Reverse;
use std::cmp::{Eq, Ord, Ordering};
use std::collections::BinaryHeap;
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct S<T>((T, usize));
impl<T: PartialEq> Eq for S<T> {}
impl<T> Ord for S<T>
where
    T: PartialOrd + PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// verified https://atcoder.jp/contests/arc064/submissions/18080990
fn wdijkstra<T>(g: &Vec<Vec<(usize, T)>>, s: usize) -> Vec<Option<T>>
where
    T: PartialOrd + PartialEq + Zero + Clone + Copy,
{
    let n = g.len();
    let mut q = BinaryHeap::new();
    q.push(Reverse(S((T::zero(), s))));
    let mut dl = vec![None; n];
    while let Some(Reverse(S((d, u)))) = q.pop() {
        for &(v, w) in &g[u] {
            let nd = d + w;
            if dl[v] == None || dl[v].unwrap() > nd {
                dl[v] = Some(nd);
                q.push(Reverse(S((nd, v))));
            }
        }
    }
    dl
}

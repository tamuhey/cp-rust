// 全方位木dp
// rerooting
// verified https://atcoder.jp/contests/abc160/submissions/19199609

use num::Zero;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

struct ReRoot<'a, T> {
    tree: &'a [Vec<usize>],
    memo: BTreeMap<(usize, usize), T>,
}

impl<'a, T> ReRoot<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Zero,
{
    fn new(tree: &'a [Vec<usize>]) -> Self {
        let mut ret = Self {
            tree,
            memo: BTreeMap::new(),
        };
        ret.calc();
        ret
    }
    fn calc(&mut self) {
        self.get(0, 0);
        self.reroot(0, 0);
    }
    fn reroot(&mut self, p: usize, i: usize) {
        if i != p {
            let ret = self.get(p, p) - self.get(p, i);
            self.memo.insert((i, p), ret);
            let r = self.get(p, i);
            let ret = r + ret;
            self.memo.insert((i, i), ret);
        };
        for &c in &self.tree[i] {
            if c != p {
                self.reroot(i, c);
            }
        }
    }
    fn get(&mut self, p: usize, i: usize) -> T {
        if let Some(&ret) = self.memo.get(&(p, i)) {
            return ret;
        }
        let mut ret = T::zero();
        for &j in self.tree[i].iter() {
            if p != j {
                ret = ret + self.get(i, j);
            }
        }
        self.memo.insert((p, i), ret);
        ret
    }
}

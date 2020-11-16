trait Counter<T> {
    fn push(&mut self, k: T) {
        self.pushv(k, 1)
    }
    fn pop(&mut self, k: &T) -> bool {
        self.popv(k, 1)
    }
    fn merge(&mut self, other: &mut Self);
    fn diff(&mut self, other: &Self);
    fn popv(&mut self, k: &T, v: usize) -> bool;
    fn pushv(&mut self, k: T, v: usize);
    fn from_iter(iter: impl Iterator<Item = T>) -> Self;
}

use std::collections::HashMap;
use std::hash::Hash;
impl<T> Counter<T> for HashMap<T, usize>
where
    T: Hash + Eq,
{
    fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        let mut ret = Self::new();
        for v in iter {
            ret.pushv(v, 1);
        }
        ret
    }
    fn pushv(&mut self, k: T, v: usize) {
        *self.entry(k).or_default() += v;
    }
    fn popv(&mut self, k: &T, v: usize) -> bool {
        if let Some(c) = self.get_mut(&k) {
            *c = c.saturating_sub(v);
            if *c == 0 {
                self.remove(&k);
            }
            true
        } else {
            false
        }
    }
    fn merge(&mut self, other: &mut Self) {
        for (k, v) in other.drain() {
            *self.entry(k).or_default() += v;
        }
    }
    fn diff(&mut self, other: &Self) {
        for (k, v) in other.iter() {
            self.popv(k, *v);
        }
    }
}

use std::cmp::Ord;
use std::collections::BTreeMap;
impl<T> Counter<T> for BTreeMap<T, usize>
where
    T: Ord + Copy,
{
    fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        let mut ret = Self::new();
        for v in iter {
            ret.pushv(v, 1);
        }
        ret
    }
    fn pushv(&mut self, k: T, v: usize) {
        *self.entry(k).or_default() += v;
    }
    fn popv(&mut self, k: &T, v: usize) -> bool {
        if let Some(c) = self.get_mut(&k) {
            *c = c.saturating_sub(v);
            if *c == 0 {
                self.remove(&k);
            }
            true
        } else {
            false
        }
    }
    fn merge(&mut self, other: &mut Self) {
        for (&k, &v) in other.iter() {
            *self.entry(k).or_default() += v;
        }
    }
    fn diff(&mut self, other: &Self) {
        for (k, v) in other.iter() {
            self.popv(k, *v);
        }
    }
}

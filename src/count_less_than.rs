// verified https://atcoder.jp/contests/abc174/submissions/15650411
// RangeTree 領域木
use std::cmp::Ordering::Less;
pub struct CountLessThan<T> {
    dat: Vec<Vec<T>>,
    n: usize,
}

impl<T: Ord + Copy> CountLessThan<T> {
    pub fn new(v: Vec<T>) -> Self {
        let n = v.len();
        let mut dat = vec![vec![]; 2 * n];
        for (i, &vi) in v.iter().enumerate() {
            dat[n + i].push(vi);
        }
        // merge
        for k in (1..n).rev() {
            dat[k] = dat[2 * k].clone();
            let mut tmp = dat[2 * k + 1].clone();
            dat[k].append(&mut tmp);
            dat[k].sort();
        }
        Self { dat: dat, n: n }
    }
    pub fn get(&self, l: usize, r: usize, query: T) -> usize {
        let mut l = l + self.n;
        let mut r = r + self.n;
        let mut countr = 0;
        let mut countl = 0;
        let cmp = |x: &T| match x.cmp(&query) {
            Equal => Less,
            d => d,
        };
        while l < r {
            if l & 1 != 0 {
                countl += self.dat[l].binary_search_by(cmp).unwrap_or_else(|x| x);
                l += 1;
            }
            if r & 1 != 0 {
                countr += self.dat[r - 1].binary_search_by(cmp).unwrap_or_else(|x| x);
            }
            l >>= 1;
            r >>= 1;
        }
        countl + countr
    }
}

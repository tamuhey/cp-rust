// Lowest Common Ancestor
// Verified: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4858117#1
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::cmp::{max, min};
use std::collections::*;
use std::i64;
use std::io::{stdin, Read};
use std::usize;

#[allow(unused_macros)]
macro_rules! parse {
    ($it: ident ) => {};
    ($it: ident, ) => {};
    ($it: ident, $var:ident : $t:tt $($r:tt)*) => {
        let $var = parse_val!($it, $t);
        parse!($it $($r)*);
    };
    ($it: ident, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = parse_val!($it, $t);
        parse!($it $($r)*);
    };
    ($it: ident, $var:ident $($r:tt)*) => {
        let $var = parse_val!($it, usize);
        parse!($it $($r)*);
    };
}

#[allow(unused_macros)]
macro_rules! parse_val {
    ($it: ident, [$t:tt; $len:expr]) => {
        (0..$len).map(|_| parse_val!($it, $t)).collect::<Vec<_>>();
    };
    ($it: ident, ($($t: tt),*)) => {
        ($(parse_val!($it, $t)),*)
    };
    ($it: ident, u1) => {
        $it.next().unwrap().parse::<usize>().unwrap() -1
    };
    ($it: ident, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn msb(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 1 {
        n >>= 1;
        ret += 1;
    }
    ret
}

struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

type Tree = Vec<Vec<usize>>;

impl LowestCommonAncestor {
    fn new(tree: &Tree, root: usize) -> Self {
        let n = tree.len();
        let k = msb(n - 1) + 1;
        let mut parent = vec![vec![!0; k]; n];
        let mut depth = vec![!0; n];
        let mut stack = vec![root];
        depth[root] = 0;
        while let Some(v) = stack.pop() {
            for &c in &tree[v] {
                parent[c][0] = v;
                depth[c] = depth[v] + 1;
                stack.push(c);
            }
        }
        for i in 1..k {
            for j in 0..n {
                let p = parent[j][i - 1];
                if p != !0 {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }
        Self { parent, depth }
    }

    fn get(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] > self.depth[v] {
            (v, u)
        } else {
            (u, v)
        };
        // 同じ深さまで移動する
        if self.depth[u] < self.depth[v] {
            let mut l = self.depth[v] - self.depth[u];
            let mut cur = 0;
            while l > 0 {
                if l & 1 == 1 {
                    v = self.parent[v][cur];
                }
                cur += 1;
                l >>= 1;
            }
        }
        if u == v {
            return u;
        }
        // 2分探索で共通祖先の1個先を見つける
        for cur in (0..self.parent[0].len()).rev() {
            if self.parent[u][cur] != self.parent[v][cur] {
                u = self.parent[u][cur];
                v = self.parent[v][cur];
            }
        }
        self.parent[u][0]
    }
}

fn solve(s: &str) {
    let mut it = s.split_whitespace();
    parse!(it, n: usize);
    let mut tree = vec![vec![]; n];
    for i in 0..n {
        parse!(it, k: usize, c: [usize; k]);
        tree[i] = c;
    }
    let lca = LowestCommonAncestor::new(&tree, 0);
    parse!(it, q: usize);
    for _ in 0..q {
        parse!(it, u: usize, v: usize);
        println!("{}", lca.get(u, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let s = "
        ";
        solve(s);
    }
}
trait ChMinMax {
    fn chmin(&mut self, other: Self);
    fn chmax(&mut self, other: Self);
}
impl<T> ChMinMax for T
where
    T: PartialOrd,
{
    fn chmin(&mut self, other: Self) {
        if *self > other {
            *self = other
        }
    }
    fn chmax(&mut self, other: Self) {
        if *self < other {
            *self = other
        }
    }
}

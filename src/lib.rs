#![feature(test)]
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod bellman_ford;
pub mod binary_tree;
pub mod bit;
pub mod comb;
pub mod combinations;
pub mod combmod;
pub mod complex;
pub mod fft;
pub mod matrix;
pub mod maxflow;
pub mod mint;
pub mod permutations;
pub mod rand;
pub mod segment_tree;
pub mod union_find;
pub mod vec_binary_search;



fn primes(mut n: usize) -> std::collections::HashMap<usize, usize> {
    let mut ret = std::collections::HashMap::new();
    let n0 = n;
    let mut cur = 2;
    while cur * cur <= n0 {
        if n % cur != 0 {
            cur += 1;
            continue;
        }
        let mut count = 0;
        while n % cur == 0 {
            n /= cur;
            count += 1;
        }
        ret.insert(cur, count);
        cur += 1;
    }
    if n > 1 {
        ret.insert(n, 1);
    }
    ret
}
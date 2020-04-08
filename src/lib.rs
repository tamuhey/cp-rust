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
pub mod combmod;
pub mod complex;
pub mod fft;
pub mod matrix;
pub mod maxflow;
pub mod mint;
pub mod permutation;
pub mod rand;
pub mod segment_tree;
pub mod union_find;
pub mod vec_binary_search;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod comb;
pub mod combmod;
pub mod heap_perm;
pub mod mint;
pub mod rand;
pub mod vec_binary_search;

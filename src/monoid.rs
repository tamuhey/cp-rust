use ac_library_rs::segtree::Monoid;
use std::convert::Infallible;
use std::marker::PhantomData;
pub struct Xor(Infallible, PhantomData<fn() -> usize>);
impl Monoid for Xor {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

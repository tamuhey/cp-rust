use num::{Bounded, One, Zero};
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::{Add, Mul};
pub trait Monoid {
    type S;
    fn id() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

// #[derive(Copy, Clone, Debug)]
// pub struct Sum<T>(PhantomData<T>);

// impl<T> Monoid for Sum<T>
// where
//     T: Zero + Add<Output = T> + Copy,
// {
//     type S = T;
//     fn id() -> Self::S {
//         T::zero()
//     }
//     fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
//         *lhs + *rhs
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct Product<T>(pub T);

// impl<T: Copy + One + Mul<Output = T>> Monoid for Product<T> {
//     fn id() -> Self {
//         Self(T::one())
//     }

//     fn op(l: &Self, r: &Self) -> Self {
//         Self(l.0 * r.0)
//     }
// }

// impl<T> From<T> for Product<T> {
//     fn from(v: T) -> Self {
//         Product(v)
//     }
// }

// #[derive(Copy, Clone, Debug)]
// pub struct Max<T>(pub T);

// impl<T: Copy + Ord + Bounded> Monoid for Max<T> {
//     fn id() -> Self {
//         Self(<T as Bounded>::min_value())
//     }

//     fn op(l: &Self, r: &Self) -> Self {
//         Self(l.0.max(r.0))
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct Min<T>(Infallible, PhantomData<T>);

impl<T> Monoid for Min<T>
where
    T: Copy + Ord + Bounded,
{
    type S = T;
    fn id() -> Self::S {
        <T as Bounded>::max_value()
    }

    fn op(l: &Self::S, r: &Self::S) -> Self::S {
        (*l).min(*r)
    }
}

use num::{Bounded, One, Zero};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

enum Void {}

pub trait Monoid {
    type S;
    fn id() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

pub struct Sum<T>(Void, PhantomData<T>);

impl<T> Monoid for Sum<T>
where
    T: Zero + Add<Output = T> + Copy,
{
    type S = T;
    fn id() -> Self::S {
        T::zero()
    }
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
        *lhs + *rhs
    }
}

pub struct Product<T>(Void, PhantomData<T>);

impl<T> Monoid for Product<T>
where
    T: Copy + One + Mul<Output = T>,
{
    type S = T;
    fn id() -> Self::S {
        T::one()
    }

    fn op(l: &Self::S, r: &Self::S) -> Self::S {
        *l * *r
    }
}

pub struct Max<T>(Void, PhantomData<T>);

impl<T> Monoid for Max<T>
where
    T: Copy + Ord + Bounded,
{
    type S = T;
    fn id() -> Self::S {
        <T as Bounded>::min_value()
    }

    fn op(l: &Self::S, r: &Self::S) -> Self::S {
        *l.max(r)
    }
}

pub struct Min<T>(Void, PhantomData<T>);

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

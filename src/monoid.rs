use num::{Bounded, One, Zero};
use std::ops::{Add, Mul};
pub trait Monoid {
    fn id() -> Self;
    fn op(lhs: &Self, rhs: &Self) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct Sum<T>(T);

impl<T> Monoid for Sum<T>
where
    T: Zero + Add<Output = T> + Copy,
{
    fn id() -> Self {
        Self(T::zero())
    }
    fn op(lhs: &Self, rhs: &Self) -> Self {
        Self(lhs.0 + rhs.0)
    }
}

impl<T> From<T> for Sum<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Product<T>(pub T);

impl<T: Copy + One + Mul<Output = T>> Monoid for Product<T> {
    fn id() -> Self {
        Self(T::one())
    }

    fn op(l: &Self, r: &Self) -> Self {
        Self(l.0 * r.0)
    }
}

impl<T> From<T> for Product<T> {
    fn from(v: T) -> Self {
        Product(v)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Max<T>(pub T);

impl<T: Copy + Ord + Bounded> Monoid for Max<T> {
    fn id() -> Self {
        Self(<T as Bounded>::min_value())
    }

    fn op(l: &Self, r: &Self) -> Self {
        Self(l.0.max(r.0))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Min<T>(pub T);

impl<T: Copy + Ord + Bounded> Monoid for Min<T> {
    fn id() -> Self {
        Self(<T as Bounded>::max_value())
    }

    fn op(l: &Self, r: &Self) -> Self {
        Self(l.0.min(r.0))
    }
}

// verified: https://atcoder.jp/contests/abc009/submissions/16843843
use num::{One, Zero};
use std::ops::{Add, BitAnd, BitXor, Mul};
// And Xor rig
#[derive(Debug, Copy, Clone)]
struct AX<T>(pub T);

impl<T> Add<AX<T>> for AX<T>
where
    T: BitXor<Output = T> + Copy,
{
    type Output = AX<T::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl<T> Mul<AX<T>> for AX<T>
where
    T: BitAnd<Output = T> + Copy,
{
    type Output = AX<T::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl<T> From<T> for AX<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}
use std::str::FromStr;
impl<T> FromStr for AX<T>
where
    T: FromStr,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = T::from_str(s)?;
        Ok(Self(v))
    }
}

use std::fmt::Display;
impl<T> Display for AX<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Zero for AX<T>
where
    T: Zero + Copy + PartialEq + BitXor,
    Self: Add<Output = Self>,
{
    fn zero() -> Self {
        Self(T::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for AX<usize> {
    fn one() -> Self {
        Self(!0)
    }
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

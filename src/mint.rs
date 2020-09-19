use num::Zero;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;
pub const MOD: usize = 1000_000_007;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Mint<T>(pub T);

impl Zero for Mint<usize> {
    fn zero() -> Self {
        Self(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Mint<usize> {
    pub fn pow(self, exp: usize) -> Self {
        let mut exp = exp;
        let mut v = self;
        let mut ret = Mint(1);
        while exp > 0 {
            if exp & 1 == 1 {
                ret *= v;
            }
            v *= v;
            exp >>= 1;
        }
        ret
    }
    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}

impl<T: Display + Copy + PartialEq> Display for Mint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FromStr + Copy + PartialEq> FromStr for Mint<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = T::from_str(s)?;
        Ok(Mint(v))
    }
}

impl Add<usize> for Mint<usize> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Mint((self.0 + rhs) % MOD)
    }
}

impl Add for Mint<usize> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0
    }
}

impl AddAssign<usize> for Mint<usize> {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl AddAssign for Mint<usize> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<usize> for Mint<usize> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        let rhs = rhs % MOD;
        let lhs = self.0 + if self.0 < rhs { MOD } else { 0 };
        Mint(lhs - rhs)
    }
}

impl Sub for Mint<usize> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0
    }
}

impl SubAssign<usize> for Mint<usize> {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl SubAssign for Mint<usize> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<usize> for Mint<usize> {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Mint((self.0 * rhs) % MOD)
    }
}

impl Mul for Mint<usize> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs.0
    }
}

impl MulAssign<usize> for Mint<usize> {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}

impl MulAssign for Mint<usize> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<usize> for Mint<usize> {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        self * Mint(rhs).pow(MOD - 2)
    }
}

impl Div for Mint<usize> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self / rhs.0
    }
}

impl DivAssign<usize> for Mint<usize> {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
    }
}
impl DivAssign for Mint<usize> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

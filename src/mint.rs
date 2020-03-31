pub mod mint {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
    use std::str::FromStr;
    pub const MOD: usize = 1000_000_007;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Mint<T: Copy + PartialEq>(pub T);

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

    impl<T: FromStr + Copy + PartialEq> FromStr for Mint<T> {
        type Err = T::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let v = T::from_str(s)?;
            Ok(Self(v))
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
}
#[cfg(test)]
mod test {
    use super::mint::*;
    use std::str::FromStr;

    #[test]
    fn test_fromstr() {
        let s = "12";
        assert_eq!(Mint::<usize>::from_str(s).unwrap().0, 12);
        let s = "O";
        Mint::<usize>::from_str(s).unwrap_err();
    }

    #[quickcheck]
    fn pow(x: usize, y: u32) -> bool {
        match x.checked_pow(y) {
            None => true,
            Some(z) => z % MOD == Mint(x).pow(y as usize).0,
        }
    }

    #[quickcheck]
    fn inv(x: usize) {
        if x == 0 {
            return;
        }
        assert_eq!((Mint(x).inv() * x).0, 1, "x: {}", x);
    }

    #[quickcheck]
    fn add_usize(x: usize, y: usize) -> bool {
        match x.checked_add(y) {
            None => true,
            Some(z) => z % MOD == (Mint(x) + y).0,
        }
    }
    #[quickcheck]
    fn add(x: usize, y: usize) -> bool {
        match x.checked_add(y) {
            None => true,
            Some(z) => z % MOD == (Mint(x) + Mint(y)).0,
        }
    }
    #[quickcheck]
    fn add_assign_usize(x: usize, y: usize) -> bool {
        let mut mx = Mint(x);
        match x.checked_add(y) {
            None => true,
            Some(z) => {
                mx += y;
                z % MOD == mx.0
            }
        }
    }

    #[quickcheck]
    fn add_assign(x: usize, y: usize) -> bool {
        let mut mx = Mint(x);
        let my = Mint(y);
        match x.checked_add(y) {
            None => true,
            Some(z) => {
                mx += my;
                z % MOD == mx.0
            }
        }
    }

    #[quickcheck]
    fn sub_usize(x: usize, y: usize) -> bool {
        let z = ((x as i64) - (y as i64)) % (MOD as i64);
        let z = if z < 0 { z + (MOD as i64) } else { z };
        z as usize == (Mint(x) - y).0
    }

    #[quickcheck]
    fn sub(x: usize, y: usize) -> bool {
        let z = ((x as i64) - (y as i64)) % (MOD as i64);
        let z = if z < 0 { z + (MOD as i64) } else { z };
        z as usize == (Mint(x) - Mint(y)).0
    }

    #[quickcheck]
    fn sub_assign_usize(x: usize, y: usize) -> bool {
        let z = ((x as i64) - (y as i64)) % (MOD as i64);
        let z = if z < 0 { z + (MOD as i64) } else { z };
        let mut mx = Mint(x);
        mx -= y;
        z as usize == mx.0
    }

    #[quickcheck]
    fn sub_assign(x: usize, y: usize) -> bool {
        let z = ((x as i64) - (y as i64)) % (MOD as i64);
        let z = if z < 0 { z + (MOD as i64) } else { z };
        let mut mx = Mint(x);
        mx -= Mint(y);
        z as usize == mx.0
    }

    #[quickcheck]
    fn mul_usize(x: usize, y: usize) -> bool {
        match x.checked_mul(y) {
            None => true,
            Some(z) => z % MOD == (Mint(x) * y).0,
        }
    }

    #[quickcheck]
    fn mul(x: usize, y: usize) -> bool {
        match x.checked_mul(y) {
            None => true,
            Some(z) => z % MOD == (Mint(x) * Mint(y)).0,
        }
    }

    #[quickcheck]
    fn div_usize(x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }
        let mz = Mint(x) / y;
        (mz * y).0 == x % MOD
    }

    #[quickcheck]
    fn div(x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }
        let mz = Mint(x) / Mint(y);
        (mz * y).0 == x % MOD
    }

    #[quickcheck]
    fn div_assign_usize(x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }
        let mut mx = Mint(x);
        mx /= y;
        (mx * y).0 == x % MOD
    }

    #[quickcheck]
    fn div_assign(x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }
        let mut mx = Mint(x);
        mx /= Mint(y);
        (mx * y).0 == x % MOD
    }
}

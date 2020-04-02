pub mod complex {
    use std::iter::Sum;
    use std::ops::{Add, Div, Mul, Neg, Sub};
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    pub struct Complex<T> {
        pub re: T,
        pub im: T,
    }

    impl<T> Complex<T> {
        pub fn new(re: T, im: T) -> Self {
            Complex { re: re, im: im }
        }
    }

    impl<T: Neg<Output = T>> Complex<T> {
        pub fn conj(self) -> Self {
            Complex {
                re: self.re,
                im: -self.im,
            }
        }
    }

    impl Complex<f64> {
        pub fn exp(self) -> Self {
            let sc = self.im.sin_cos();
            Complex { re: sc.1, im: sc.0 } * self.re.exp()
        }
    }

    impl<T: Neg<Output = T>> Neg for Complex<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }

    impl<T: Add> Add for Complex<T> {
        type Output = Complex<T::Output>;
        fn add(self, rhs: Self) -> Self::Output {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    impl<T: Add + Default> Add<T> for Complex<T> {
        type Output = Complex<T::Output>;
        fn add(self, rhs: T) -> Self::Output {
            self + Complex::new(rhs, T::default())
        }
    }

    impl<T: Sub> Sub for Complex<T> {
        type Output = Complex<T::Output>;
        fn sub(self, rhs: Self) -> Self::Output {
            Complex {
                re: self.re - rhs.re,
                im: self.im - rhs.im,
            }
        }
    }

    impl<T: Sub + Default> Sub<T> for Complex<T> {
        type Output = Complex<T::Output>;
        fn sub(self, rhs: T) -> Self::Output {
            self - Complex::new(rhs, T::default())
        }
    }

    impl<T> Mul for Complex<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Complex {
                re: self.re * rhs.re - self.im * rhs.im,
                im: self.re * rhs.im + self.im * rhs.re,
            }
        }
    }

    impl<T> Mul<T> for Complex<T>
    where
        T: Mul + Copy,
    {
        type Output = Complex<T::Output>;
        fn mul(self, rhs: T) -> Self::Output {
            Complex {
                re: self.re * rhs,
                im: self.im * rhs,
            }
        }
    }

    impl<T> Div for Complex<T>
    where
        T: Div<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Copy
            + Neg<Output = T>,
    {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            let den = rhs.re * rhs.re + rhs.im * rhs.im;
            let num = self * rhs.conj();
            Complex {
                re: num.re / den,
                im: num.im / den,
            }
        }
    }

    impl<T> Div<T> for Complex<T>
    where
        T: Div + Copy,
    {
        type Output = Complex<T::Output>;
        fn div(self, rhs: T) -> Self::Output {
            Complex {
                re: self.re / rhs,
                im: self.im / rhs,
            }
        }
    }

    impl<T: Default + Add<Output = T>> Sum for Complex<T> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut ret = Complex::<T>::default();
            for i in iter {
                ret = ret + i;
            }
            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::complex::*;
    #[test]
    fn test_add() {
        let a = Complex { re: 1, im: 2 };
        let b = Complex { re: 10, im: 23 };
        assert_eq!(Complex { re: 11, im: 25 }, a + b);
        assert_eq!(Complex { re: 3, im: 2 }, a + 2);
    }

    #[test]
    fn test_sub() {
        let a = Complex { re: 10, im: 23 };
        let b = Complex { re: 1, im: 21 };
        assert_eq!(Complex { re: 9, im: 2 }, a - b);
        assert_eq!(Complex { re: 8, im: 23 }, a - 2);
    }

    #[test]
    fn test_mul() {
        let a = Complex { re: 43, im: 12 };
        let b = Complex { re: 3, im: 4 };
        assert_eq!(Complex { re: 81, im: 208 }, a * b);
    }

    #[test]
    fn test_div() {
        let a = Complex {
            re: 43f64,
            im: 12f64,
        };
        let b = Complex { re: 3f64, im: 4f64 };
        assert_eq!(
            Complex {
                re: 7.08,
                im: -5.44
            },
            a / b
        );
    }

    fn nearly_equal(x: f64, y: f64) -> bool {
        (x - y).abs() < 1e-8
    }

    #[test]
    fn test_exp() {
        let c = Complex { re: 3f64, im: 4f64 };
        let e = c.exp();
        assert!(nearly_equal(e.re, -13.12878308146215808032755));
        assert!(nearly_equal(e.im, -15.2007844630679545622034810233));
    }
}

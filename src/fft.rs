pub mod fft {
    use super::super::complex::complex::*;
    use std::ops::{Add, Div, Mul, Sub};
    #[cfg(test)]
    fn slow_dft(f: Vec<f64>) -> Vec<Complex<f64>> {
        let n = f.len();
        let i = Complex { re: 0f64, im: 1f64 };
        (0..n)
            .map(|t| {
                let tf = t as f64;
                (0..n)
                    .map(|x| {
                        let xf = x as f64;
                        (-i * xf * tf / (2f64 * std::f64::consts::PI * (n as f64))) * f[x]
                    })
                    .sum()
            })
            .collect()
    }
}

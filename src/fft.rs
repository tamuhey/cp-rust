use num::Complex;
use std::f64::{self, consts::PI};

fn msb(mut n: usize) -> usize {
    if n == 0 {
        0
    } else {
        let mut ret = 0;
        while n > 1 {
            n >>= 1;
            ret += 1;
        }
        ret
    }
}

fn pad_to_pow2(f: &mut Vec<f64>) {
    let n = f.len();
    if n.count_ones() <= 1 {
        return;
    }
    let m = msb(n);
    let p = 1 << (m + 1);
    f.resize(p, 0.);
}

fn fft(f: &mut Vec<f64>) -> Vec<Complex<f64>> {
    let n = f.len();
    match n {
        0 => unimplemented!(),
        1 => {
            let re = f[0];
            vec![Complex { re, im: 0. }]
        }
        _ => {
            pad_to_pow2(f);
            let n = f.len();
            let mut f0: Vec<_> = f.iter().cloned().step_by(2).collect();
            let mut f1: Vec<_> = f.iter().skip(1).cloned().step_by(2).collect();
            let f0 = fft(&mut f0);
            let f1 = fft(&mut f1);
            let theta = 2. * PI / (n as f64);
            let zeta = Complex {
                re: theta.cos(),
                im: theta.sin(),
            };
            let mut pow_zeta: Complex<_> = 1f64.into();
            let mut ret = vec![0f64.into(); n];
            for i in 0..n {
                ret[i] = f0[i % (n / 2)] + pow_zeta * f1[i % (n / 2)];
                pow_zeta *= zeta;
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: Complex<f64> = Complex { re: 0f64, im: 1f64 };

    fn slow_dft(f: &mut Vec<f64>) -> Vec<Complex<f64>> {
        pad_to_pow2(f);
        let n = f.len();
        (0..n)
            .map(|t| {
                let tf = t as f64;
                (0..n)
                    .map(|x| {
                        let xf = x as f64;
                        (-I * xf * tf / (2f64 * std::f64::consts::PI * (n as f64))) * f[x]
                    })
                    .sum()
            })
            .collect()
    }

    #[quickcheck]
    fn test_fft(mut f: Vec<f64>) {
        assert_eq!(slow_dft(&mut f), fft(&mut f));
    }
}

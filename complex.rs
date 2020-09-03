use num::complex::Complex;
use std::cmp::Ordering;

fn argcmp(a: &Complex<i64>, b: &Complex<i64>) -> Ordering {
    use std::cmp::Ordering::*;
    if a.im == 0 && a.re >= 0 {
        return Less;
    }
    if b.im == 0 && b.re >= 0 {
        return Greater;
    }
    let c = cross(*a, *b);
    if a.im >= 0 {
        if c < 0 && b.im >= 0 {
            Greater
        } else {
            Less
        }
    } else {
        if c > 0 && b.im < 0 {
            Less
        } else {
            Greater
        }
    }
}

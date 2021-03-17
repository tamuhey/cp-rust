// Verified: https://yukicoder.me/submissions/555843
use num::{One, Zero};
use std::ops::{Add, Mul};

struct Matrix<T>(Vec<Vec<T>>);

fn idmat<T>(n: usize) -> Vec<Vec<T>>
where
    T: One + Zero + Clone,
{
    let mut ret = vec![vec![T::zero(); n]; n];
    for i in 0..n {
        ret[i][i] = T::one();
    }
    ret
}

fn matmul<T>(a: &[Vec<T>], b: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Add<T> + Mul<Output = T> + Zero + Copy,
{
    assert_eq!(a[0].len(), b.len());
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|k| {
                    (0..b.len())
                        .map(|j| (a[i][j] * b[j][k]))
                        .fold(T::zero(), |x, y| (x + y))
                })
                .collect()
        })
        .collect()
}

fn matpow<T>(a: &Vec<Vec<T>>, mut p: usize) -> Vec<Vec<T>>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Copy + One,
{
    let n = a.len();
    let mut ret = idmat(n);
    let mut cur = a.clone();
    while p > 0 {
        if p & 1 == 1 {
            ret = matmul(&ret, &cur);
        }
        cur = matmul(&cur, &cur);
        p >>= 1;
    }
    ret
}

fn matmulmod(a: &[Vec<usize>], b: &[Vec<usize>], m: usize) -> Vec<Vec<usize>> {
    assert_eq!(a[0].len(), b.len());
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|k| {
                    (0..b.len())
                        .map(|j| (a[i][j] * b[j][k]) % m)
                        .fold(0, |x, y| (x + y) % m)
                })
                .collect()
        })
        .collect()
}

fn matpowmod(a: &Vec<Vec<usize>>, mut p: usize, m: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut ret = idmat(n);
    let mut cur = a.clone();
    while p > 0 {
        if p & 1 == 1 {
            ret = matmulmod(&ret, &cur, m);
        }
        cur = matmulmod(&cur, &cur, m);
        p >>= 1;
    }
    ret
}

pub fn matvecmod(a: &[Vec<usize>], x: &[usize], m: usize) -> Vec<usize> {
    (0..a.len())
        .map(|i| {
            (0..x.len())
                .map(|j| a[i][j] * x[j])
                .fold(0, |i, j| (i + j) % m)
        })
        .collect()
}
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::cmp::{max, min};
use std::collections::*;
use std::fmt::Write as _Write;
use std::i64;
use std::io::{stdin, BufWriter, Read, Write};
use std::usize;

#[allow(unused_macros)]
macro_rules! parse {
    ($it: ident ) => {};
    ($it: ident, ) => {};
    ($it: ident, $var:ident : $t:tt $($r:tt)*) => {
        let $var = parse_val!($it, $t);
        parse!($it $($r)*);
    };
    ($it: ident, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = parse_val!($it, $t);
        parse!($it $($r)*);
    };
    ($it: ident, $var:ident $($r:tt)*) => {
        let $var = parse_val!($it, usize);
        parse!($it $($r)*);
    };
}

#[allow(unused_macros)]
macro_rules! parse_val {
    ($it: ident, [$t:tt; $len:expr]) => {
        (0..$len).map(|_| parse_val!($it, $t)).collect::<Vec<_>>();
    };
    ($it: ident, ($($t: tt),*)) => {
        ($(parse_val!($it, $t)),*)
    };
    ($it: ident, u1) => {
        $it.next().unwrap().parse::<usize>().unwrap() -1
    };
    ($it: ident, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

macro_rules! debug_print {
    ($($arg:tt)*) => ({
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    })
}

fn diff(x: usize, y: usize) -> usize {
    max(x, y) - min(x, y)
}

fn solve(s: &str) {
    let mut it = s.split_whitespace();
    loop {
        parse!(it, m: usize, n: usize, d: usize);
        if (m, n, d) == (0, 0, 0) {
            return;
        }
        parse!(it, s: [[u8; m]; n]);
        let s: Vec<Vec<_>> = s
            .into_iter()
            .map(|line| line.into_iter().map(|x| x == 1).collect())
            .collect();
        let id = |i: usize, j: usize| i * m + j;
        let md = |(x0, y0): (usize, usize), (x1, y1)| diff(x0, x1) + diff(y0, y1);
        let nm = n * m;
        let mut a = {
            let mut a = vec![vec![false; nm]; nm];
            for x0 in 0..n {
                for y0 in 0..m {
                    let x = id(x0, y0);
                    a[x][x] = true;
                    for x1 in 0..n {
                        for y1 in 0..m {
                            if md((x0, y0), (x1, y1)) == d {
                                let y = id(x1, y1);
                                a[x][y] = true;
                            }
                        }
                    }
                }
            }
            a
        };
        let b = {
            let mut b = vec![false; nm];
            for i in 0..n {
                for j in 0..m {
                    b[id(i, j)] = s[i][j];
                }
            }
            b
        };
        // let mut y = vec![];
        // let yyy = vec![false, true, false, true, false, true, false, true, false];
        // for i in 0..nm {
        //     y.push((0..nm).map(|j| a[i][j] ^ yyy[j]).fold(false, |x, y| x ^ y))
        // }
        // debug_print!("{:?}", y);
        // debug_print!("{:?}", b);
        if let Some(rank) = linear_eq_f2(&mut a, &b) {
            // debug_print!("{:?}", rank);
            // for line in a.iter() {
            //     // debug_print!("{:?}", line.iter().map(|x| *x as usize).collect::<Vec<_>>());
            // }

            // let mut y = vec![];
            // for i in 0..nm {
            //     let mut yi = false;
            //     for j in 0..nm {
            //         yi ^= a[i][j] & a[i][nm];
            //     }
            //     y.push(yi);
            // }
            // debug_print!("{:?}", y);
            // debug_print!("{:?}", b);
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
    }
}


/// returns rank
/// answer is stored in last columns in `a`
/// verified: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1308
fn linear_eq_f2(a: &mut [Vec<bool>], b: &[bool]) -> Option<usize> {
    let h = a.len();
    let w = a[0].len();
    for i in 0..h {
        a[i].push(b[i]);
    }
    let rank = gauss_jordan_f2(a);
    if (rank..h).any(|i| a[i][w]) {
        return None;
    }
    Some(rank)
}

/// verified: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1308
fn gauss_jordan_f2(mat: &mut [Vec<bool>]) -> usize {
    let mut rank = 0usize;
    let h = mat.len();
    let w = mat[0].len();
    for col in 0..w - 1 {
        if let Some(pivot) = (rank..h).filter(|&i| mat[i][col]).next() {
            mat.swap(rank, pivot);
            for row in (0..h).filter(|&i| i != rank) {
                if mat[row][col] {
                    for i in 0..w {
                        mat[row][i] ^= mat[rank][i];
                    }
                }
            }
            rank += 1;
        }
    }
    rank
}


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

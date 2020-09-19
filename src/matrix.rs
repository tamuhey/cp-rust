// Verified: https://yukicoder.me/submissions/555843
use num::{One, Zero};

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

pub fn matvecmod(a: Vec<Vec<usize>>, x: Vec<usize>, m: usize) -> Vec<usize> {
    (0..a.len())
        .map(|i| {
            (0..x.len())
                .map(|j| a[i][j] * x[j])
                .fold(0, |i, j| (i + j) % m)
        })
        .collect()
}

pub fn matmulmod(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, m: usize) -> Vec<Vec<usize>> {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| {
                    (0..b.len())
                        .map(|k| ((a[i][k] % m) * (b[k][j] % m)))
                        .fold(0, |i, j| (i + j) % m)
                })
                .collect()
        })
        .collect()
}

pub fn matpowmod(a: &Vec<Vec<usize>>, n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut n = n;
    let mut ret: Vec<Vec<usize>> = (0..a.len())
        .map(|i| {
            let mut a = vec![0; a.len()];
            a[i] = 1;
            a
        })
        .collect();
    let mut cur = a.clone();
    while n > 0 {
        if n & 1 == 1 {
            ret = matmulmod(&ret, &cur, m);
        }
        cur = matmulmod(&cur, &cur, m);
        n >>= 1;
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

fn perm_part<T: Clone>(n: usize, v: &mut [T], all: &mut Vec<Vec<T>>) {
    if n == 1 {
        all.push(v.to_vec());
        return;
    }
    for i in 0..n {
        perm_part(n - 1, v, all);
        if n % 2 == 0 {
            v.swap(i, n - 1);
        } else {
            v.swap(0, n - 1);
        }
    }
}

pub fn perm<T: Clone>(v: &Vec<T>) -> Vec<Vec<T>> {
    let mut v = v.clone();
    let mut all = vec![];
    perm_part(v.len(), &mut v, &mut all);
    all
}

#[test]
fn test_perm() {
    let v = vec![1, 2, 3];
    let ret = [
        [1, 2, 3],
        [2, 1, 3],
        [3, 2, 1],
        [1, 3, 2],
        [2, 3, 1],
        [3, 1, 2],
    ];
    let all = perm(&v);
    assert!((0..ret.len()).all(|i| (0..all.len()).any(|j| ret[i].to_vec() == all[j])));
    assert_eq!(all.len(), ret.len());
}

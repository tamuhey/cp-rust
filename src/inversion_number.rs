// 転倒数
fn get_inversion_number(v: &[usize]) -> Vec<usize> {
    let n = v.len();
    let mut ft = fenwicktree::FenwickTree::new(n, 0);
    let mut ret = vec![0; n];
    for (i, &vi) in v.iter().enumerate() {
        ret[i] = ft.sum(vi, n);
        ft.add(vi, 1);
    }
    ret
}

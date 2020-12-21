use ac_library_rs::fenwicktree;
// 転倒数, inv_number
pub fn get_inversion_number(v: &[usize]) -> Vec<usize> {
    let n = v.len();
    let mut ft = fenwicktree::FenwickTree::new(n, 0);
    let mut ret = vec![0; n];
    for (i, &vi) in v.iter().enumerate() {
        ret[i] = ft.sum(vi, n);
        ft.add(vi, 1);
    }
    ret
}

// 分割統治法による転倒数
// verified: https://atcoder.jp/contests/arc075/submissions/18917697
use std::mem::swap;
fn inv_number<T>(a: &mut [T]) -> usize
where
    T: Clone + Default + Ord,
{
    fn _inv_number<T>(a: &mut [T], buf: &mut Vec<T>) -> usize
    where
        T: Ord,
    {
        if a.len() <= 1 {
            0
        } else {
            let n = a.len();
            let (v, w) = a.split_at_mut(n / 2);
            let mut ret = _inv_number(v, buf) + _inv_number(w, buf);

            let mut lv = 0;
            let ev = v.len();
            let mut lw = 0;
            let ew = w.len();
            let mut lb = 0;
            while lv < ev || lw < ew {
                let nv = if lv < ev && lw < ew {
                    v[lv] <= w[lw]
                } else {
                    lv < ev
                };
                if nv {
                    swap(&mut v[lv], &mut buf[lb]);
                    lv += 1;
                } else {
                    swap(&mut w[lw], &mut buf[lb]);
                    lw += 1;
                    ret += ev - lv;
                }
                lb += 1;
            }
            for i in 0..a.len() {
                swap(&mut a[i], &mut buf[i]);
            }
            ret
        }
    }
    _inv_number(a, &mut vec![T::default(); a.len()])
}

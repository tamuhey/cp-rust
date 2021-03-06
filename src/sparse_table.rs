// sparse table: https://www.geeksforgeeks.org/sparse-table/
// 区間の最小値をもとめる

fn msb(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 1 {
        n >>= 1;
        ret += 1;
    }
    ret
}

struct SparseTable<T> {
    table: Vec<Vec<T>>,
}

use num::Bounded;
use std::cmp::min;
impl<T> SparseTable<T>
where
    T: Copy + Bounded + Ord,
{
    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        let m = msb(n) + 1;
        let max_value = T::max_value();
        let mut table = vec![vec![max_value; n]; m];
        table[0] = a.to_vec();
        for i in 1..m {
            for j in 0..n {
                let k = j + (1 << (i - 1));
                let mut tmp = table[i - 1][j];
                if k < n {
                    tmp = min(tmp, table[i - 1][k]);
                }
                table[i][j] = tmp;
            }
        }

        Self { table }
    }
    pub fn get(&self, l: usize, r: usize) -> T {
        assert!(l < r);
        let d = r - l;
        let k = msb(d);
        min(self.table[k][l], self.table[k][r - (1 << k)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let a = vec![7, 2, 3, 0, 5];
        let expected = vec![
            vec![7, 2, 3, 0, 5],
            vec![2, 2, 0, 0, 5],
            vec![0, 0, 0, 0, 5],
        ];
        let actual = SparseTable::new(&a).table;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_msb() {
        assert_eq!(msb(2), 1);
        assert_eq!(msb(1), 0);
    }
    #[test]
    fn get_handmade() {
        let a = vec![0, 1, 0];
        let st = SparseTable::new(&a);
        assert_eq!(st.get(0, 2), 0);
    }
    #[quickcheck]
    fn get_quick(a: Vec<usize>) {
        let st = SparseTable::new(&a);
        let n = a.len();
        for l in 0..n {
            for r in (l + 1)..n {
                assert_eq!(
                    st.get(l, r),
                    *a[l..r].iter().min().unwrap(),
                    "\nl: {}, r: {}, table: {:?}",
                    l,
                    r,
                    st.table
                );
            }
        }
    }
}

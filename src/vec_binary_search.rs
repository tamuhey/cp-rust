fn binary_search_left<T: Ord>(v: &Vec<T>, x: T) -> usize {
    let mut l: isize = -1;
    let mut r: isize = v.len() as isize;
    while r - l > 1 {
        let m = (r + l) / 2;
        if v[m as usize] < x {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_search_left() {
        let cases = vec![
            (vec![0, 1, 2, 3], 4, 4),
            (vec![0, 1, 2, 3], 0, 0),
            (vec![0, 1, 2, 3], 1, 1),
            (vec![0, 1, 1, 3], 1, 1),
            (vec![0, 1, 1, 3], 0, 0),
            (vec![0, 0, 1, 1, 3], 0, 0),
            (vec![0, 1, 1, 3], 3, 3),
            (vec![0, 1, 1, 3], 4, 4),
            (vec![0, 0, 3, 4, 8], 1, 2),
        ];
        for (v, x, expected) in cases {
            let actual = binary_search_left(&v, x);
            if actual != expected {
                panic!(
                    "failed: actual {}, expected {}, {:?}",
                    actual,
                    expected,
                    (v, x)
                );
            }
        }
    }
}

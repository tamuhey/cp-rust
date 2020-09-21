/// 拡張ユークリッド互除法
/// ax + by = c
/// verified: https://atcoder.jp/contests/acl1/submissions/16916747
fn ext_gcd(a: u128, b: u128) -> (u128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, y, x) = ext_gcd(b, a % b);
    // (c, x, y)
    (d, x, y - ((a / b) as i128) * x)
}

use std::collections::HashMap;
pub fn factor(mut n: usize) -> HashMap<usize, usize> {
    let mut ret = std::collections::HashMap::new();
    let n0 = n;
    let mut cur = 2;
    while cur * cur <= n0 {
        if n % cur != 0 {
            cur += 1;
            continue;
        }
        let mut count = 0;
        while n % cur == 0 {
            n /= cur;
            count += 1;
        }
        ret.insert(cur, count);
        cur += 1;
    }
    if n > 1 {
        ret.insert(n, 1);
    }
    ret
}

pub fn num_divs(facts: &HashMap<usize, usize>) -> usize {
    facts.iter().map(|(_, v)| v + 1).product()
}

#[test]
fn test_num_divs() {
    assert_eq!(num_divs(&factor(1)), 1);
    assert_eq!(num_divs(&factor(2)), 2);
    assert_eq!(num_divs(&factor(4)), 3);
    assert_eq!(num_divs(&factor(8)), 4);
    assert_eq!(num_divs(&factor(6)), 4);
    assert_eq!(num_divs(&factor(57)), 4);
    assert_eq!(num_divs(&factor(60)), 12);
}

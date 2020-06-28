pub fn get_primes(mut n: usize) -> std::collections::HashMap<usize, usize> {
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

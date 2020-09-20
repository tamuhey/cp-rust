fn msb(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 1 {
        n >>= 1;
        ret += 1;
    }
    ret
}

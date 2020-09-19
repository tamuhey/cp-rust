fn next_comb(s: usize) -> usize {
    let x = s & (!s + 1);
    let y = s + x;
    (((s & !y) / x) >> 1) | y
}

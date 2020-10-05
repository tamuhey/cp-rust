// Grundy数を求める例
// https://www.hackerrank.com/contests/5-days-of-game-theory/challenges/day-1-a-chessboard-game/problem
fn get_grundy(memo: &mut Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let k = memo.len();
    if memo[i][j] != !0 {
        return memo[i][j];
    }
    let mut s = HashSet::new();
    for &(ni, nj) in &[
        (i.wrapping_sub(2), j.wrapping_sub(1)),
        (i.wrapping_sub(2), j + 1),
        (i + 1, j.wrapping_sub(2)),
        (i.wrapping_sub(1), j.wrapping_sub(2)),
    ] {
        if ni < k && nj < k {
            s.insert(get_grundy(memo, ni, nj));
        }
    }
    let ret = (0..).find(|i| !s.contains(&i)).unwrap();
    memo[i][j] = ret;
    ret
}

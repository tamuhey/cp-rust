/// verified https://atcoder.jp/contests/joi2008ho/submissions/16993316
/// ボードの連結数を数える
fn count_connected(seen: &mut [Vec<bool>]) -> usize {
    let w = seen.len();
    let h = seen[0].len();
    let mut count = 0;
    for i in 0..w {
        for j in 0..h {
            if !seen[i][j] {
                count += 1;
                seen[i][j] = true;
                let mut stack = vec![(i, j)];
                while let Some((x, y)) = stack.pop() {
                    for &(i, j) in &[
                        (x.wrapping_add(!0), y),
                        (x + 1, y),
                        (x, y.wrapping_add(!0)),
                        (x, y + 1),
                    ] {
                        if i >= w || j >= h || seen[i][j] {
                            continue;
                        }
                        stack.push((i, j));
                        seen[i][j] = true;
                    }
                }
            }
        }
    }
    count
}

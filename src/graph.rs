// verified: https://atcoder.jp/contests/acl1/submissions/16942490
// それぞれの連結成分の個数をカウントする
fn count_component_sizes(g: &[Vec<usize>]) -> Vec<usize> {
    let mut seen = HashSet::new();
    let n = g.len();
    let mut ret = vec![0; n];
    let mut stack = vec![0];
    for i in 0..n {
        if ret[i] != 0 {
            continue;
        }
        seen.clear();
        stack.clear();
        stack.push(i);
        seen.insert(i);
        while let Some(i) = stack.pop() {
            for &c in &g[i] {
                if !seen.contains(&c) {
                    stack.push(c);
                    seen.insert(c);
                }
            }
        }
        let m = seen.len();
        for &i in &seen {
            ret[i] = m;
        }
    }
    ret
}

fn count_components_unordered(g: &[Vec<usize>]) -> usize {
    let mut seen = HashSet::new();
    let mut ret = 0;
    let mut stack = vec![0];
    let n = g.len();
    for i in 0..n {
        if seen.contains(&i) {
            continue;
        }
        stack.clear();
        stack.push(i);
        while let Some(i) = stack.pop() {
            for &c in &g[i] {
                if !seen.contains(&c) {
                    stack.push(c);
                    seen.insert(c);
                }
            }
        }
        ret += 1;
    }
    ret
}

/// returns a number of nodes in one clored group if graph can be bi-colored
/// verified: https://atcoder.jp/contests/code-festival-2017-qualb/submissions/17282096
fn bicolor(g: &[Vec<usize>]) -> Option<usize> {
    let n = g.len();
    let mut color = vec![!0; n];
    let mut stack = vec![(0, 0)];
    while let Some((u, c)) = stack.pop() {
        color[u] = c;
        for &v in &g[u] {
            match color[v] {
                d if d == !0 => stack.push((v, 1 - c)),
                d if d == c => return None,
                _ => (),
            }
        }
    }
    Some(color.iter().filter(|x| **x == 0).count())
}

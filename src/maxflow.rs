use std::cmp::min;
fn dfs(
    g: &Vec<Vec<usize>>,
    s: usize,
    t: usize,
    parents: &mut Vec<usize>,
    seen: &mut Vec<bool>,
) -> bool {
    seen[s] = true;
    if s == t {
        true
    } else {
        let n = g.len();
        for v in 0..n {
            if !seen[v] && g[s][v] > 0 && dfs(g, v, t, parents, seen) {
                parents[v] = s;
                return true;
            }
        }
        false
    }
}

fn calc_maxflow(g: &Vec<Vec<usize>>, s: usize, t: usize) -> usize {
    let n = g.len();
    let mut ret = 0;
    let mut parents = vec![0; n];
    let mut seen = vec![false; n];
    let mut rg = g.clone();
    while dfs(&rg, s, t, &mut parents, &mut seen) {
        let mut d = std::usize::MAX;
        let mut v = t;
        while v != s {
            let u = parents[v];
            d = min(d, rg[u][v]);
            v = u;
        }
        ret += d;
        let mut v = t;
        while v != s {
            let u = parents[v];
            rg[u][v] -= d;
            rg[v][u] += d;
            v = u;
        }
        seen = vec![false; n];
    }
    ret
}

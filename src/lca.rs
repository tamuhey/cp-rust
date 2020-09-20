// Lowest Common Ancestor
// Verified: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4858117#1
// Verified: https://atcoder.jp/contests/abc014/tasks/abc014_4
fn msb(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 1 {
        n >>= 1;
        ret += 1;
    }
    ret
}

struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

type Tree = Vec<Vec<usize>>;

impl LowestCommonAncestor {
    fn new(tree: &Tree, root: usize) -> Self {
        let n = tree.len();
        let k = msb(n - 1) + 1;
        let mut parent = vec![vec![!0; k]; n];
        let mut depth = vec![!0; n];
        let mut stack = vec![root];
        depth[root] = 0;
        let mut seen = vec![false; n];
        seen[root] = true;
        while let Some(v) = stack.pop() {
            for &c in &tree[v] {
                if !seen[c] {
                    parent[c][0] = v;
                    depth[c] = depth[v] + 1;
                    stack.push(c);
                    seen[c] = true;
                }
            }
        }
        for i in 1..k {
            for j in 0..n {
                let p = parent[j][i - 1];
                if p != !0 {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }
        Self { parent, depth }
    }

    fn get(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] > self.depth[v] {
            (v, u)
        } else {
            (u, v)
        };
        // 同じ深さまで移動する
        if self.depth[u] < self.depth[v] {
            let mut l = self.depth[v] - self.depth[u];
            let mut cur = 0;
            while l > 0 {
                if l & 1 == 1 {
                    v = self.parent[v][cur];
                }
                cur += 1;
                l >>= 1;
            }
        }
        if u == v {
            return u;
        }
        // 2分探索で共通祖先の1個先を見つける
        for cur in (0..self.parent[0].len()).rev() {
            if self.parent[u][cur] != self.parent[v][cur] {
                u = self.parent[u][cur];
                v = self.parent[v][cur];
            }
        }
        self.parent[u][0]
    }
}

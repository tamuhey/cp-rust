pub struct TreeLCA<'a> {
    g: &'a Vec<Vec<usize>>,
    parents: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
    logn: usize,
    n: usize,
}

impl<'a> TreeLCA<'a> {
    pub fn new(g: &'a Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let logn = (n as f64).log2().ceil() as usize;
        let parents = vec![vec![None; n]; logn];
        let depth = vec![0; n];
        let mut ret = TreeLCA {
            g: g,
            parents: parents,
            depth: depth,
            logn: logn,
            n: n,
        };
        ret.dfs(0, None, 0);
        for k in 1..logn {
            for i in 0..n {
                match ret.parents[k][i] {
                    Some(p) => ret.parents[k][i] = ret.parents[k][p],
                    None => (),
                }
            }
        }
        ret
    }
    pub fn get(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] > self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        let mut d = self.depth[u] - self.depth[v];
        let mut cur = 0;
        while d > 0 {
            if (d >> cur) & 1 == 1 {
                u = self.parents[cur][u].unwrap();
            }
            d = self.depth[u] - self.depth[v];
            cur += 1;
        }
        if u == v {
            return u;
        }
        for i in (0..self.logn).rev() {
            if self.parents[i][u] != self.parents[i][v] {
                u = self.parents[i][u].unwrap();
                v = self.parents[i][v].unwrap();
            }
        }
        self.parents[0][u].unwrap()
    }
    fn dfs(&mut self, u: usize, p: Option<usize>, d: usize) {
        self.parents[0][u] = p;
        self.depth[u] = d;
        for &v in self.g[u].iter() {
            if p == None || p.unwrap() != v {
                self.dfs(v, Some(u), d + 1);
            }
        }
    }
}

// トポロジカルソート
// topological sort
// verified: https://atcoder.jp/contests/typical90/submissions/23821025
#[derive(Debug)]
struct TopoSort {
    g: Vec<Vec<usize>>,
}

impl TopoSort {
    fn from_edges(edges: &[(usize, usize)], n: usize) -> Self {
        let mut adj = vec![vec![]; n + 1];
        for &(i, j) in edges.iter() {
            adj[i].push(j);
        }
        for i in 0..n {
            adj[n].push(i); // sentinel
        }
        Self { g: adj }
    }

    fn get_indegrees(&self) -> Vec<usize> {
        let n = self.g.len();
        let mut indegrees = vec![0; n];
        for i in 0..n {
            for &j in self.g[i].iter() {
                indegrees[j] += 1;
            }
        }
        indegrees
    }

    fn toposort(&self) -> Option<Vec<usize>> {
        let n = self.g.len();
        let mut ret = vec![];
        let mut marks = vec![None; n];
        fn visit(
            g: &[Vec<usize>],
            marks: &mut [Option<bool>],
            ret: &mut Vec<usize>,
            i: usize,
        ) -> bool {
            marks[i] = Some(false);
            for &j in g[i].iter() {
                match marks[j] {
                    Some(false) => return false,
                    Some(true) => (),
                    None => {
                        if !visit(g, marks, ret, j) {
                            return false;
                        }
                    }
                }
            }
            marks[i] = Some(true);
            ret.push(i);
            true
        }
        let x = visit(&self.g, &mut marks, &mut ret, n - 1);
        if !x {
            return None;
        }
        ret.reverse();
        Some(ret)
    }

    fn get_all_sorts(&self, paths: &mut Vec<Vec<usize>>, lim: usize) -> bool {
        // Get at most lim topological sorts
        // Returns `lim` sorts are found or not, or failed

        struct Sol<'a> {
            g: &'a [Vec<usize>],
            n: usize,
            seen: Vec<bool>,
            indeg: Vec<usize>,
            paths: &'a mut Vec<Vec<usize>>,
            path: Vec<usize>,
            lim: usize,
            que: Vec<usize>,
        }

        impl<'a> Sol<'a> {
            fn f(&mut self) -> bool {
                if self.path.len() == self.n {
                    self.paths.push(self.path.clone());
                    return true;
                }
                if self.que.is_empty() {
                    return false;
                }

                for idx in (0..self.que.len()).rev() {
                    if self.paths.len() == self.lim {
                        break;
                    }
                    let i = self.que[idx];
                    assert_eq!(self.indeg[i], 0);
                    self.que.remove(idx);

                    for &j in self.g[i].iter() {
                        self.indeg[j] -= 1;
                        if self.indeg[j] == 0 {
                            self.que.push(j);
                        }
                    }
                    self.path.push(i);
                    self.seen[i] = true;

                    let p = self.f();
                    if p {
                    } else {
                        return false;
                    }

                    // backtrack
                    for &j in self.g[i].iter() {
                        if self.indeg[j] == 0 {
                            self.que.pop();
                        }
                        self.indeg[j] += 1;
                    }
                    self.path.pop();
                    self.que.insert(idx, i);
                }
                true
            }
        }

        let n = self.g.len();
        let seen = vec![false; n];
        let indeg = self.get_indegrees();
        let que = vec![n - 1];
        let mut sol = Sol {
            g: &self.g,
            n,
            seen,
            paths,
            lim,
            indeg,
            path: Default::default(),
            que,
        };
        sol.f()
    }
}

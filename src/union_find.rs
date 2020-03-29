struct UnionFind {
    parents: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = (0..n).collect();
        UnionFind { parents: par }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        let par = self.parents[x];
        self.parents[x] = self.root(par);
        self.parents[x]
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return;
        }
        self.parents[rx] = ry;
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        rx == ry
    }
}

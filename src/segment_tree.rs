// verified in https://atcoder.jp/contests/arc008/submissions/10187379
trait Monoid {
    fn unit() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
}

struct SegTree<T: Monoid + Clone> {
    dat: Vec<T>,
    n: usize,
}

impl<T: Monoid + Copy> SegTree<T> {
    pub fn new(n: usize) -> Self {
        let dat = vec![T::unit(); n << 1];
        SegTree { dat: dat, n: n }
    }
    pub fn update(&mut self, k: usize, v: T) {
        let mut k = k + self.n;
        self.dat[k] = v;
        while {
            k >>= 1;
            k > 0
        } {
            self.dat[k] = T::add(self.dat[k << 1], self.dat[k << 1 | 1]);
        }
    }
    pub fn get(&self, a: usize, b: usize) -> T {
        let mut a = a + self.n;
        let mut b = b + self.n;
        let mut va = T::unit();
        let mut vb = T::unit();
        while a < b {
            if a & 1 != 0 {
                va = T::add(va, self.dat[a]);
                a += 1;
            }
            if b & 1 != 0 {
                vb = T::add(self.dat[b - 1], vb);
            }
            a >>= 1;
            b >>= 1;
        }
        T::add(va, vb)
    }
}

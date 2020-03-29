// verified in https://atcoder.jp/contests/chokudai_S001/submissions/10200651
struct BIT<T> {
    dat: Vec<T>,
    unit: T,
}

impl<T: std::ops::Add<Output = T> + Clone> BIT<T> {
    fn new(n: usize, unit: T) -> Self {
        let mut dat = vec![unit.clone(); n];
        BIT {
            dat: dat,
            unit: unit.clone(),
        }
    }
    fn add(&mut self, k: usize, v: T) {
        let mut x = k;
        let n = self.dat.len();
        while x < n {
            self.dat[x] = self.dat[x].clone() + v.clone();
            x |= x + 1;
        }
    }
    fn sum(&self, k: usize) -> T {
        let mut ret = self.unit.clone();
        if k == 0 {
            return ret;
        }
        let mut x = k - 1;
        loop {
            ret = self.dat[x].clone() + ret.clone();
            x = x & (x + 1);
            if x == 0 {
                break;
            };
            x -= 1;
        }
        ret
    }
}

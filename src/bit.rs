// verified in https://atcoder.jp/contests/chokudai_S001/submissions/10200651
pub struct BIT<T> {
    pub dat: Vec<T>,
    pub unit: T,
}

impl<T: std::ops::Add<Output = T> + Clone> BIT<T> {
    pub fn new(n: usize, unit: T) -> Self {
        let dat = vec![unit.clone(); n];
        BIT {
            dat: dat,
            unit: unit.clone(),
        }
    }
    pub fn add(&mut self, k: usize, v: T) {
        let mut x = k;
        let n = self.dat.len();
        while x < n {
            self.dat[x] = self.dat[x].clone() + v.clone();
            x |= x + 1;
        }
    }
    pub fn sum(&self, k: usize) -> T {
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

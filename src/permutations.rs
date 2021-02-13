/// genarate permutation based on Heap's algorithm
struct HeapPermute<T> {
    v: Vec<T>,
    c: Vec<usize>,
    i: usize,
}

impl<T> HeapPermute<T> {
    fn new(v: Vec<T>) -> Self {
        let n = v.len();
        Self {
            v,
            c: vec![0; n],
            i: 0,
        }
    }

    fn next(&mut self) -> bool {
        let n = self.v.len();
        while self.i < n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.v.swap(0, self.i)
                } else {
                    self.v.swap(self.c[self.i], self.i)
                }
                self.c[self.i] += 1;
                self.i = 0;
                return true;
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        false
    }
}

#[test]
fn test_perm() {
    let v: Vec<_> = (0..7).collect();
    let mut p = HeapPermute::new(v);
    let mut seen: Vec<Vec<usize>> = vec![];
    while p.next() {
        for v in seen.iter() {
            assert_ne!(v, &p.v);
        }
        seen.push(p.v.clone());
    }
}

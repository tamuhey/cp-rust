struct Perm<T> {
    v: Vec<T>,
    cur: usize,
    tails: Option<Box<Perm<T>>>,
    finish: bool,
}

impl<T: Clone> Perm<T> {
    fn new(v: Vec<T>) -> Self {
        let mut w = v.clone();
        w.remove(0);
        let tails = if w.len() > 0 {
            Some(Box::new(Perm::new(w)))
        } else {
            None
        };
        Perm {
            v: v,
            cur: 0,
            tails: tails,
            finish: false,
        }
    }
}

impl<T: Clone> Iterator for Perm<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            return None;
        }
        if let &mut Some(ref mut tails) = &mut self.tails {
            if let Some(mut v) = tails.next() {
                v.insert(0, self.v[self.cur].clone());
                return Some(v);
            }
        } else {
            self.finish = true;
            return Some(self.v.clone());
        }

        self.cur += 1;
        if self.cur == self.v.len() {
            self.finish = true;
            return None;
        }
        let mut w = self.v.clone();
        w.remove(self.cur);
        self.tails = Some(Box::new(Perm::new(w)));
        self.next()
    }
}

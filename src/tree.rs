use std::collections::VecDeque;

fn tree_diameter(g: &[Vec<usize>]) -> (usize, usize, usize) {
    fn f(s: usize, g: &[Vec<usize>]) -> (usize, usize) {
        let n = g.len();
        let mut q = VecDeque::new();
        q.push_back((n, s, 1));
        while let Some((p, u, d)) = q.pop_front() {
            for &v in g[u].iter() {
                if v != p {
                    q.push_back((u, v, d + 1));
                }
            }
            if q.len() == 0 {
                return (u, d);
            }
        }
        unreachable!();
    }
    let (s, _) = f(0, g);
    let (t, d) = f(s, g);
    (s, t, d)
}

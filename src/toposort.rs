// topological sort
// トポロジカルソート
// あんまテストされてない
fn toposort(g: &[Vec<usize>]) -> Option<Vec<usize>> {
    #[derive(Clone)]
    enum Mark {
        Unseen,
        Seen,
        Completed,
    }
    use Mark::*;
    let mut ret = vec![];
    let n = g.len();
    let mut mark = vec![Unseen; n];

    fn visit(g: &[Vec<usize>], i: usize, mark: &mut [Mark], ret: &mut Vec<usize>) -> bool {
        match mark[i] {
            Seen => false,
            Unseen => {
                mark[i] = Seen;
                for &j in g[i].iter() {
                    if !visit(g, j, mark, ret) {
                        return false;
                    }
                }
                mark[i] = Completed;
                ret.push(i);
                true
            }
            _ => true,
        }
    }

    for i in 0..n {
        if let Unseen = mark[i] {
            if !visit(g, i, &mut mark, &mut ret) {
                return None;
            }
        }
    }
    ret.reverse();
    Some(ret)
}

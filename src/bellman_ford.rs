pub fn bellman_ford(g: &Vec<Vec<(usize, isize)>>, source: usize) -> Result<Vec<isize>, ()> {
    let n = g.len();
    let mut ret = vec![std::isize::MAX / 2; n];
    ret[source] = 0;
    for _ in 0..(n - 1) {
        for (i, e) in g.iter().enumerate() {
            for &(j, c) in e.iter() {
                if ret[j] > ret[i] + c {
                    ret[j] = ret[i] + c
                }
            }
        }
    }
    for (i, e) in g.iter().enumerate() {
        for &(j, c) in e.iter() {
            if ret[j] > ret[i] + c {
                return Err(());
            }
        }
    }
    Ok(ret)
}

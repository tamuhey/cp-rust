// z-algorithm
// 文字列sについて、sと`s[i:]`の最長共通接頭辞の長さを返す
// ref: https://snuke.hatenablog.com/entry/2014/12/03/214243
// varified: https://atcoder.jp/contests/arc055/submissions/17727872
fn z_algorithm(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n];
    ret[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1
        }
        ret[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    ret
}

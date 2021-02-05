// kmp法
#[derive(Debug, Clone)]
struct KMP<T> {
    word: Vec<T>,
    table: Vec<usize>,
}
impl<T> KMP<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn new(word: Vec<T>) -> Self {
        let n = word.len();
        let mut table = vec![0usize; n];
        table[0] = !0usize;
        let mut i = 2;
        let mut j = 0;
        while i < n {
            if word[i - 1] == word[j] {
                table[i] = j + 1;
                i += 1;
                j += 1;
            } else if j > 0 {
                j = table[j];
            } else {
                table[i] = 0;
                i += 1;
            }
        }
        Self { word, table }
    }
    fn search(&self, s: &[T]) -> Option<usize> {
        let mut m = 0;
        let mut i = 0;
        let n = s.len();
        while m + i < n {
            if self.word[i] == s[m.wrapping_add(i)] {
                i = i.wrapping_add(1);
                if i == self.word.len() {
                    return Some(m);
                }
            } else {
                m = m.wrapping_add(i).wrapping_sub(self.table[i]);
                if i > 0 {
                    i = self.table[i];
                }
            }
        }
        None
    }
}

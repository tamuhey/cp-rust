/// Implementation of AVL tree
#[cfg(test)]
extern crate quickcheck;
use std::cmp::Ordering;
use std::default::Default;
use std::iter::{FromIterator, Iterator};
use std::mem;

#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}
use BinaryTree::*;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
    balance_factor: i8,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Empty
    }
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    pub fn insert(&mut self, value: T) -> bool {
        self.add(value).0
    }

    fn add(&mut self, value: T) -> (bool, bool) {
        // returns: (inserted, deepened)
        let ret = match *self {
            Empty => {
                let node = Node {
                    value: value,
                    left: Empty,
                    right: Empty,
                    balance_factor: 0,
                };
                *self = NonEmpty(Box::new(node));
                (true, true)
            }
            NonEmpty(ref mut node) => match node.value.cmp(&value) {
                Ordering::Equal => (false, false),
                Ordering::Less => {
                    let (inserted, deepened) = node.right.add(value);
                    if deepened {
                        let ret = match node.balance_factor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.balance_factor += 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                }
                Ordering::Greater => {
                    let (inserted, deepened) = node.left.add(value);
                    if deepened {
                        let ret = match node.balance_factor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.balance_factor -= 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                }
            },
        };
        self.balance();
        ret
    }

    fn balance(&mut self) {
        match *self {
            Empty => (),
            NonEmpty(ref mut node) => match node.balance_factor {
                -2 => {
                    let lf = node.left.node().unwrap().balance_factor;
                    if lf == -1 || lf == 0 {
                        let (a, b) = if lf == -1 { (0, 0) } else { (-1, 1) };
                        self.rotate_right();
                        self.node().unwrap().right.node().unwrap().balance_factor = a;
                        self.node().unwrap().balance_factor = b;
                    } else if lf == 1 {
                        let (a, b) = match node
                            .left
                            .node()
                            .unwrap()
                            .right
                            .node()
                            .unwrap()
                            .balance_factor
                        {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };
                        node.left.rotate_left();
                        self.rotate_right();
                        self.node().unwrap().right.node().unwrap().balance_factor = a;
                        self.node().unwrap().left.node().unwrap().balance_factor = b;
                        self.node().unwrap().balance_factor = 0;
                    } else {
                        unreachable!()
                    }
                }
                2 => {
                    let lf = node.right.node().unwrap().balance_factor;
                    if lf == 1 || lf == 0 {
                        let (a, b) = if lf == 1 { (0, 0) } else { (1, -1) };
                        self.rotate_left();
                        self.node().unwrap().left.node().unwrap().balance_factor = a;
                        self.node().unwrap().balance_factor = b;
                    } else if lf == -1 {
                        let (a, b) = match node
                            .right
                            .node()
                            .unwrap()
                            .left
                            .node()
                            .unwrap()
                            .balance_factor
                        {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        };
                        node.right.rotate_right();
                        self.rotate_left();
                        self.node().unwrap().left.node().unwrap().balance_factor = a;
                        self.node().unwrap().right.node().unwrap().balance_factor = b;
                        self.node().unwrap().balance_factor = 0;
                    } else {
                        unreachable!()
                    }
                }
                _ => (),
            },
        }
    }

    fn node(&mut self) -> Option<&mut Node<T>> {
        match *self {
            Empty => None,
            NonEmpty(ref mut v) => Some(v),
        }
    }

    fn right(&mut self) -> &mut Self {
        match *self {
            Empty => panic!("call on empty tree"),
            NonEmpty(ref mut node) => &mut node.right,
        }
    }

    fn left(&mut self) -> &mut Self {
        match *self {
            Empty => panic!("call on empty tree"),
            NonEmpty(ref mut node) => &mut node.left,
        }
    }

    fn rotate_right(&mut self) {
        let mut v = mem::take(self);
        let mut left = mem::take(v.left());
        let left_right = mem::take(left.right());
        *v.left() = left_right;
        *left.right() = v;
        *self = left;
    }

    fn rotate_left(&mut self) {
        let mut v = mem::take(self);
        let mut right = mem::take(v.right());
        let right_left = mem::take(right.left());
        *v.right() = right_left;
        *right.left() = v;
        *self = right;
    }

    fn depth(&self) -> usize {
        match *self {
            Empty => 0,
            NonEmpty(ref v) => std::cmp::max(v.left.depth(), v.right.depth()) + 1,
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            Empty => 0,
            NonEmpty(ref v) => 1 + v.left.len() + v.right.len(),
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    #[cfg(test)]
    fn value(&self) -> Option<&T> {
        match *self {
            Empty => None,
            NonEmpty(ref v) => Some(&v.value),
        }
    }

    #[cfg(test)]
    fn print(&self, prefix: &str)
    where
        T: std::fmt::Debug,
    {
        match *self {
            Empty => (),
            NonEmpty(ref v) => {
                println!("{} {:?} {:?}", prefix, v.value, v.balance_factor);
                v.left.print(&(prefix.to_string() + "L"));
                v.right.print(&(prefix.to_string() + "R"))
            }
        }
    }
}

pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>,
}

impl<T> IntoIter<T> {
    pub fn new(tree: BinaryTree<T>) -> Self {
        let stack = match tree {
            Empty => vec![],
            NonEmpty(v) => vec![v],
        };
        IntoIter { stack: stack }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(mut u) => match mem::take(&mut u.left) {
                    Empty => {
                        if let NonEmpty(right) = u.right {
                            self.stack.push(right);
                        };
                        return Some(u.value);
                    }
                    NonEmpty(left) => {
                        self.stack.push(u);
                        self.stack.push(left);
                    }
                },
            }
        }
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct Iter<'a, T> {
    stack: Vec<(&'a Node<T>, bool)>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(tree: &'a BinaryTree<T>) -> Self {
        let stack = match tree {
            Empty => vec![],
            NonEmpty(v) => vec![(v.as_ref(), false)],
        };
        Iter { stack: stack }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some((u, flag)) => {
                    if flag {
                        if let NonEmpty(ref right) = u.right {
                            self.stack.push((right, false))
                        }
                        return Some(&u.value);
                    } else {
                        self.stack.push((u, true));
                        if let NonEmpty(ref left) = u.left {
                            self.stack.push((left, false))
                        }
                    }
                }
            }
        }
    }
}

impl<T: Ord> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Empty;
        for v in iter {
            tree.insert(v);
        }
        tree
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn rotate_right() {
        let mut tree = BinaryTree::Empty;
        tree.add(3usize);
        tree.add(4usize);
        tree.add(1usize);
        tree.add(2usize);
        tree.add(0usize);
        assert_eq!(tree.left().value().unwrap(), &1);
        assert_eq!(tree.left().left().value().unwrap(), &0);
        tree.rotate_right();
        assert_eq!(tree.value().unwrap(), &1);
        assert_eq!(tree.left().value().unwrap(), &0);
        assert_eq!(tree.right().value().unwrap(), &3);
        assert_eq!(tree.right().left().value().unwrap(), &2);
    }

    #[test]
    fn rotate_left() {
        let mut tree = BinaryTree::Empty;
        tree.add(3usize);
        tree.add(1usize);
        tree.add(10usize);
        tree.add(8usize);
        tree.add(13usize);
        tree.rotate_left();
        assert_eq!(tree.value().unwrap(), &10);
        assert_eq!(tree.left().value().unwrap(), &3);
        assert_eq!(tree.left().right().value().unwrap(), &8);
    }

    fn check_height<T: Ord>(tree: BinaryTree<T>) -> bool {
        let n = tree.len();
        let h = tree.depth() as f64 - 1.;
        let l = ((n + 1) as f64).log2() - 1.;
        let r = 1.45 * ((n + 2) as f64).log2() - 1.32;
        (l <= h) & (h < r)
    }

    #[quickcheck]
    fn len(v: HashSet<usize>) -> bool {
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        tree.len() == v.len()
    }

    #[quickcheck]
    fn balance(v: HashSet<usize>) -> bool {
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        check_height(tree)
    }

    #[quickcheck]
    fn into_iter(v: HashSet<usize>) -> bool {
        let mut v: Vec<_> = v.into_iter().collect();
        v.sort();
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        let w: Vec<_> = tree.into_iter().collect();
        (0..v.len()).all(|i| v[i] == w[i])
    }

    #[quickcheck]
    fn iter(v: HashSet<usize>) -> bool {
        let mut v: Vec<_> = v.into_iter().collect();
        v.sort();
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        let w: Vec<_> = tree.iter().collect();
        (0..v.len()).all(|i| v[i] == *w[i])
    }

    #[quickcheck]
    fn from_iter(v: HashSet<usize>) {
        let n = v.len();
        let w = v.clone();
        let mut w: Vec<_> = w.into_iter().collect();
        w.sort();
        let tree: BinaryTree<_> = v.into_iter().collect();
        assert_eq!(tree.len(), n);
        for (i, a) in tree.into_iter().enumerate() {
            assert_eq!(a, w[i]);
        }
    }

    #[quickcheck]
    fn duplicates(v: Vec<i64>) -> bool {
        let tree = v.into_iter().collect::<BinaryTree<_>>();
        let mut v: Vec<_> = tree.into_iter().collect();
        let n = v.len();
        v.dedup();
        n == v.len()
    }
}

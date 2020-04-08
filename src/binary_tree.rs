/// Implementation of AVL tree
#[cfg(test)]
extern crate quickcheck;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::default::Default;
use std::iter::{FromIterator, Iterator};
use std::mem;
use std::ops::Index;

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

    pub fn iter<'a>(&'a self) -> RangeIter<'a, T, &T> {
        RangeIter::new(self, None, None)
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        match *self {
            Empty => None,
            NonEmpty(ref node) => match value.cmp(node.value.borrow()) {
                Less => node.left.get(value),
                Equal => Some(&node.value),
                Greater => node.right.get(value),
            },
        }
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
        let mut into_iter = IntoIter { stack: Vec::new() };
        into_iter.traverse_left(tree);
        into_iter
    }

    fn traverse_left(&mut self, mut tree: BinaryTree<T>) {
        while let NonEmpty(mut node) = tree {
            tree = mem::take(&mut node.left);
            self.stack.push(node);
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut node = match self.stack.pop() {
            None => return None,
            Some(node) => node,
        };
        self.traverse_left(mem::take(&mut node.right));
        Some(node.value)
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
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

impl<T: Ord> Index<&T> for BinaryTree<T> {
    type Output = T;
    fn index(&self, index: &T) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct RangeIter<'a, T, K> {
    end: Option<K>,
    stack: Vec<&'a Node<T>>,
}

impl<'a, 'b, T, K> RangeIter<'a, T, &'b K>
where
    T: Ord + Borrow<K>,
    K: ?Sized + Ord,
{
    fn new(tree: &'a BinaryTree<T>, start: Option<&'b K>, end: Option<&'b K>) -> Self {
        let mut iter = RangeIter {
            end: end,
            stack: Vec::new(),
        };
        match start {
            None => iter.traverse_left(tree),
            Some(i) => iter.traverse(tree, i),
        }
        iter
    }
    fn traverse_left(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = tree {
            self.stack.push(node);
            tree = &node.left;
        }
    }
    fn traverse(&mut self, tree: &'a BinaryTree<T>, start: &K) {
        match *tree {
            Empty => (),
            NonEmpty(ref node) => match start.cmp(node.value.borrow()) {
                Less => {
                    self.stack.push(&node);
                    self.traverse(&node.left, start);
                }
                Equal => self.stack.push(&node),
                Greater => {
                    self.traverse(&node.right, start);
                }
            },
        }
    }
}

impl<'a, 'b, T, K> Iterator for RangeIter<'a, T, &'b K>
where
    T: Ord + Borrow<K>,
    K: ?Sized + Ord,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(ref node) => match self.end {
                None => {
                    self.traverse_left(&node.right);
                    Some(&node.value)
                }
                Some(r) => match r.cmp(&node.value.borrow()) {
                    Greater => {
                        self.traverse_left(&node.right);
                        Some(&node.value)
                    }
                    _ => None,
                },
            },
        }
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

    #[quickcheck]
    fn get_and_index(v: HashSet<usize>, indices: Vec<usize>) -> bool {
        let w = v.clone();
        let tree: BinaryTree<_> = v.into_iter().collect();
        w.iter().all(|wi| tree.get(wi) != None)
            && indices.iter().all(|i| w.get(i) == tree.get(i))
            && w.iter().all(|wi| tree[wi] == *wi)
    }
}

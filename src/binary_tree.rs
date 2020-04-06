/// Implementation of AVL tree
#[cfg(test)]
extern crate quickcheck;
use std::cmp::Ordering;
use std::default::Default;
use std::iter::Iterator;
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
    T: Ord + std::fmt::Debug,
{
    pub fn add(&mut self, value: T) -> (bool, bool) {
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
    #[cfg(test)]
    fn value(&self) -> Option<&T> {
        match *self {
            Empty => None,
            NonEmpty(ref v) => Some(&v.value),
        }
    }

    #[cfg(test)]
    fn depth(&self) -> usize {
        match *self {
            Empty => 0,
            NonEmpty(ref v) => std::cmp::max(v.left.depth(), v.right.depth()) + 1,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right() {
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
    fn test_rotate_left() {
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

    #[test]
    fn test_balance_handmade() {
        let mut tree = Empty;
        tree.add(3);
        tree.add(2);
        tree.add(1);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.value().unwrap(), &2);
        assert_eq!(tree.left().value().unwrap(), &1);
        assert_eq!(tree.right().value().unwrap(), &3);
    }

    #[test]
    fn test_balance_handmade2() {
        let mut tree = Empty;
        tree.add(4);
        tree.add(2);
        tree.add(3);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.value().unwrap(), &3);
        assert_eq!(tree.left().value().unwrap(), &2);
        assert_eq!(tree.right().value().unwrap(), &4);
    }

    #[test]
    fn test_balance_handmade3() {
        let mut tree = Empty;
        let v = vec![22, 28, 90, 36];
        for &vi in v.iter() {
            tree.add(vi);
        }
    }

    #[test]
    fn test_balance_handmade4() {
        let mut tree = Empty;
        let v = vec![0, 2, 1];
        for &vi in v.iter() {
            tree.add(vi);
        }
    }

    #[test]
    fn test_balance_handmade5() {
        let mut tree = Empty;
        let v = vec![96, 39, 67, 35, 77, 3, 16, 0];
        let n = v.len();
        for &vi in v.iter() {
            tree.add(vi);
        }
        let h = tree.depth() as f64 - 1.;
        let l = ((n + 1) as f64).log2() - 1.;
        let r = 1.45 * ((n + 2) as f64).log2() - 1.32;
        assert!((l <= h) & (h < r));
    }

    #[test]
    fn test_balance_handmade6() {
        let mut tree = Empty;
        let v = vec![21, 0, 3, 16, 66, 65, 32, 44, 63, 69];
        let n = v.len();
        for &vi in v.iter() {
            println!("{}", vi);
            tree.add(vi);
            tree.print("");
            println!("");
        }
        let h = tree.depth() as f64 - 1.;
        let l = ((n + 1) as f64).log2() - 1.;
        let r = 1.45 * ((n + 2) as f64).log2() - 1.32;
        assert!((l <= h) & (h < r));
    }

    #[quickcheck]
    fn test_balance_quick(v: std::collections::HashSet<usize>) -> bool {
        let n = v.len();
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        let h = tree.depth() as f64 - 1.;
        let l = ((n + 1) as f64).log2() - 1.;
        let r = 1.45 * ((n + 2) as f64).log2() - 1.32;
        (l <= h) & (h < r)
    }
}

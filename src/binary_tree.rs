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

#[derive(Debug, Eq, PartialEq)]
enum Balanced {
    LeftLeft,
    Left,
    Equal,
    Right,
    RightRight,
}
use Balanced::*;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
    balanced: Balanced,
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
    pub fn add(&mut self, value: T) -> (bool, bool) {
        // returns: (inserted, deepened)
        let ret = match *self {
            Empty => {
                *self = NonEmpty(Box::new(Node {
                    value: value,
                    left: Empty,
                    right: Empty,
                    balanced: Balanced::Equal,
                }));
                (true, true)
            }
            NonEmpty(ref mut node) => match node.value.cmp(&value) {
                Ordering::Equal => (false, false),
                Ordering::Less => {
                    let (inserted, deepened) = node.right.add(value);
                    if deepened {
                        match node.balanced {
                            Left => {
                                node.balanced = Equal;
                                (inserted, false)
                            }
                            Equal => {
                                node.balanced = Right;
                                (inserted, true)
                            }
                            Right => {
                                node.balanced = RightRight;
                                (inserted, false)
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        (inserted, deepened)
                    }
                }
                Ordering::Greater => {
                    let (inserted, deepened) = node.left.add(value);
                    if deepened {
                        match node.balanced {
                            Left => {
                                node.balanced = LeftLeft;
                                (inserted, false)
                            }
                            Equal => {
                                node.balanced = Left;
                                (inserted, true)
                            }
                            Right => {
                                node.balanced = Equal;
                                (inserted, false)
                            }
                            _ => unreachable!(),
                        }
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
            NonEmpty(ref mut node) => match node.balanced {
                LeftLeft => {
                    node.balanced = Equal;
                    if node.left.node().unwrap().balanced == Right {
                        node.left.rotate_left();
                    }
                    self.rotate_right();
                }
                RightRight => {
                    node.balanced = Equal;
                    if node.right.node().unwrap().balanced == Left {
                        node.left.rotate_right();
                    }
                    self.rotate_left();
                }
                _ => (),
            },
        }
    }

    fn node(&self) -> Option<&Node<T>> {
        match *self {
            Empty => None,
            NonEmpty(ref v) => Some(v),
        }
    }

    fn value(&self) -> Option<&T> {
        match *self {
            Empty => None,
            NonEmpty(ref v) => Some(&v.value),
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

    #[quickcheck]
    fn test_balance_quick(v: Vec<usize>) {
        let n = v.len();
        let mut tree = Empty;
        for &vi in v.iter() {
            tree.add(vi);
        }
        let h = tree.depth() as f64;
        assert!(((n + 1) as f64).log2() - 1. <= h, "{:?}", v);
        assert!(1.45 * ((n + 2) as f64).log2() - 1.32 > h, "{:?}", v);
    }
}

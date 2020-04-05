/// Implementation of AVL tree
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
enum Balanced {
    Left,
    Equal,
    Right,
}

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
    pub fn add(&mut self, value: T) {
        match *self {
            Empty => {
                *self = NonEmpty(Box::new(Node {
                    value: value,
                    left: Empty,
                    right: Empty,
                    balanced: Balanced::Equal,
                }))
            }
            NonEmpty(ref mut node) => {
                if value < node.value {
                    node.left.add(value)
                } else if value > node.value {
                    node.right.add(value)
                }
            }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let mut tree = BinaryTree::Empty;
        tree.add(3usize);
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
}

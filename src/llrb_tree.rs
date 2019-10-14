//! Left leaning red black tree
//! Compare to BST, LLRB always has perfect balance
//!
//! The perfect balance is because when insert a new node,
//! we either put it in a node(which will not cause tree height change),
//! or split a three node, the "split" actually partition the tree from bottom to up,
//! so the tree is still balanced.
//!
//! Compare to BST, each time insert/delete a node the height will increase, it cause the BST less balanced.
//!
//! insert/search/delete O(c log(n)), c due to how we implement it
//!

use std::cmp::Ordering;

type Color = bool;
pub const RED: bool = true;
pub const BLACK: bool = false;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            color,
            left: None,
            right: None,
        }
    }
}

fn is_red<K, V>(n: &Option<Box<Node<K, V>>>) -> bool {
    match n {
        Some(n) => n.color == RED,
        None => false,
    }
}

fn rotate_left<K, V>(mut n: Box<Node<K, V>>) -> Box<Node<K, V>> {
    assert!(is_red(&n.right));
    let mut p = n.right.take().expect("red should exists");
    n.right = p.left.take();
    p.color = n.color;
    n.color = RED;
    p.left = Some(n);
    p
}

fn rotate_right<K, V>(mut n: Box<Node<K, V>>) -> Box<Node<K, V>> {
    assert!(is_red(&n.left));
    let mut p = n.left.take().expect("red should exists");
    n.left = p.right.take();
    p.color = n.color;
    n.color = RED;
    p.right = Some(n);
    p
}

fn flip_colors<K, V>(n: &mut Box<Node<K, V>>) {
    n.color = RED;
    if let Some(ref mut n) = n.left {
        n.color = BLACK;
    }
    if let Some(ref mut n) = n.right {
        n.color = BLACK;
    }
}

/// insert new node into tree
/// 1. insert key to left or right node by compare key with current node key.
/// 2. do recursive until find a none node.
/// 3. test red black color and do rotations after each insert.
fn insert<K: Ord, V>(n: Option<Box<Node<K, V>>>, k: K, v: V) -> Box<Node<K, V>> {
    if n.is_none() {
        return Box::new(Node::new(k, v, RED));
    }
    let mut n = n.unwrap();
    match k.cmp(&n.key) {
        Ordering::Less => n.left = Some(insert(n.left.take(), k, v)),
        Ordering::Greater => n.right = Some(insert(n.right.take(), k, v)),
        Ordering::Equal => n.value = v,
    }
    if !is_red(&n.left) && is_red(&n.right) {
        n = rotate_left(n)
    }
    if is_red(&n.left) && n.left.as_ref().map(|n| is_red(&n.left)).unwrap_or(false) {
        n = rotate_right(n)
    }
    if is_red(&n.left) && is_red(&n.right) {
        flip_colors(&mut n);
    }
    n
}

/// iterate tree from left to right
fn iter_sort<K, V>(n: &Box<Node<K, V>>) -> Vec<&V> {
    let mut ret = Vec::new();
    if let Some(ref l) = n.left {
        ret.extend(iter_sort(l));
    }
    ret.push(&n.value);
    if let Some(ref r) = n.right {
        ret.extend(iter_sort(r));
    }
    ret
}

/// Left leaning red black tree
pub struct LLRBTree<K, V>(Option<Box<Node<K, V>>>);

impl<K: Ord, V> LLRBTree<K, V> {
    pub fn new() -> Self {
        LLRBTree(None)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0 = Some(insert(self.0.take(), key, value));
    }

    pub fn to_vec(&self) -> Vec<&V> {
        match &self.0 {
            Some(n) => iter_sort(&n),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn try_tree_sort() {
        let mut tree = LLRBTree::new();
        let list = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        for i in list {
            tree.insert(i, i);
        }
        let sorted_list = tree.to_vec();
        assert!(is_sorted(&sorted_list));
    }

    #[test]
    fn test_rotate() {
        let n1 = Box::new(Node {
            key: "A",
            value: 1,
            left: None,
            right: None,
            color: BLACK,
        });
        let n2 = Box::new(Node {
            key: "B",
            value: 2,
            left: None,
            right: None,
            color: BLACK,
        });
        let n3 = Box::new(Node {
            key: "C",
            value: 3,
            left: None,
            right: None,
            color: BLACK,
        });
        let n4 = Box::new(Node {
            key: "D",
            value: 4,
            left: Some(n2),
            right: Some(n3),
            color: RED,
        });
        let n5 = Box::new(Node {
            key: "E",
            value: 5,
            left: Some(n1),
            right: Some(n4),
            color: BLACK,
        });
        let o_n5 = n5.clone();
        assert_eq!(rotate_right(rotate_left(n5)), o_n5);
    }
}

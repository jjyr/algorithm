//! Binary search tree
//! insert: O(n) avg: O(ln n)
//! search: O(n) avg: O(ln n)
//! delete: O(n) avg: O(ln n)
//! Notice: current implemented deletion will increase tree height

use std::cmp::Ordering;

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

fn insert<K: Ord, V>(n: Option<Box<Node<K, V>>>, key: K, value: V) -> Box<Node<K, V>> {
    if n.is_none() {
        return Box::new(Node::new(key, value));
    }
    let mut n = n.unwrap();
    match key.cmp(&n.key) {
        Ordering::Less => {
            n.left = Some(insert(n.left.take(), key, value));
        }
        Ordering::Equal => {
            n.value = value;
            return n;
        }
        Ordering::Greater => {
            n.right = Some(insert(n.right.take(), key, value));
        }
    }
    n
}

/// return new node and deleted node.
fn delete_min<K: Ord, V>(
    n: Option<Box<Node<K, V>>>,
) -> (Option<Box<Node<K, V>>>, Option<Box<Node<K, V>>>) {
    if n.is_none() {
        return (None, None);
    }
    let mut n = n.unwrap();
    if n.left.is_none() {
        return (n.right.take(), Some(n));
    }
    let (new_left, deleted) = delete_min(n.left.take());
    n.left = new_left;
    (Some(n), deleted)
}

fn delete<K: Ord, V>(n: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
    if n.is_none() {
        return None;
    }
    let mut n = n.unwrap();
    match key.cmp(&n.key) {
        Ordering::Less => {
            n.left = delete(n.left.take(), key);
        }
        Ordering::Greater => {
            n.right = delete(n.right.take(), key);
        }
        Ordering::Equal => {
            // for node with one child, return the child and delete node itself
            if n.left.is_none() {
                return n.right.take();
            } else if n.right.is_none() {
                return n.left.take();
            } else {
                // for node with two children, return node m which is min of right tree
                // the remain right nodes is always greater than min node,
                // and the left nodes is always less than the m node,
                // so the BST is still valid
                let (new, m) = delete_min(n.right);
                // we have checked the right is not none, so m must exists.
                let mut m = m.unwrap();
                m.right = new;
                m.left = n.left.take();
                return Some(m);
            }
        }
    }
    Some(n)
}

fn iter_sort<K, V: Clone>(n: &Option<Box<Node<K, V>>>, q: &mut Vec<V>) {
    let n = match n {
        Some(n) => n,
        None => return,
    };
    iter_sort(&n.left, q);
    q.push(n.value.clone());
    iter_sort(&n.right, q);
}

pub struct BinarySearchTree<K, V>(Option<Box<Node<K, V>>>);

impl<K: Ord, V: Clone> BinarySearchTree<K, V> {
    pub fn new() -> Self {
        BinarySearchTree(None)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0 = Some(insert(self.0.take(), key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut node = &self.0;
        while let Some(n) = node {
            match key.cmp(&n.key) {
                Ordering::Equal => return Some(&n.value),
                Ordering::Less => node = &n.left,
                Ordering::Greater => node = &n.right,
            }
        }
        None
    }

    pub fn delete(&mut self, key: &K) {
        self.0 = delete(self.0.take(), key)
    }

    /// return a sorted vector
    pub fn to_vec(&self) -> Vec<V> {
        let mut q = Vec::new();
        iter_sort(&self.0, &mut q);
        q
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;
    use std::collections::HashSet;

    #[test]
    fn try_sort() {
        let mut tree = BinarySearchTree::new();
        let list = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        for i in list {
            tree.insert(i, i);
        }
        let sorted_list = tree.to_vec();
        assert!(is_sorted(&sorted_list));
    }

    #[test]
    fn try_delete() {
        let mut tree = BinarySearchTree::new();
        let list = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        for &i in &list {
            tree.insert(i, i);
        }
        let mut list2 = list.clone().into_iter().collect::<HashSet<_>>();
        for i in (0..list.len()).rev().step_by(2) {
            tree.delete(&list[i]);
            list2.remove(&list[i]);
        }
        let sorted_list = tree.to_vec();
        let mut list2 = list2.into_iter().collect::<Vec<_>>();
        list2.sort();
        assert_eq!(sorted_list, list2);
    }
}

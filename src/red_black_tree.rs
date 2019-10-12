type Color = bool;
pub const RED: bool = true;
pub const BLACK: bool = false;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
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

#[cfg(test)]
mod tests {
    use super::*;

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

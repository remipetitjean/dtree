//! Simple implementation of Binary Search Node
use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Node<T: Ord> {
    /// Node structure. Empty for a null value, or a leaf with left and right children nodes
    Leaf {
        value: T,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
    Empty,
}

impl<T: Ord> Node<T> {
    /// Instantiate an empty node
    pub fn new() -> Self {
        Node::Empty
    }

    /// Create a leaf
    pub fn create(value: T) -> Self {
        Node::Leaf {
            value,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Empty),
        }
    }

    /// Inserts a node
    pub fn insert(&mut self, new_value: T) {
        match self {
            Node::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
                Ordering::Equal => return,
            },
            Node::Empty => {
                *self = Node::create(new_value);
            }
        }
    }

    /// Find a node
    pub fn find(&self, find_value: T) -> bool {
        match self {
            Node::Leaf {
                ref value,
                ref left,
                ref right,
            } => match find_value.cmp(value) {
                Ordering::Less => left.find(find_value),
                Ordering::Greater => right.find(find_value),
                Ordering::Equal => true,
            },
            Node::Empty => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find() {
        let mut t1 = Node::new();
        t1.insert(3);
        t1.insert(1);
        t1.insert(2);
        assert_eq!(true, t1.find(2));
    }
}

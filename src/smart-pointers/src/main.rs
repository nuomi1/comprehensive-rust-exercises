use std::cmp::Ordering;

/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.

impl<T: Ord> BinaryTree<T> {
    /// Creates an empty binary tree.
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    /// Inserts a value into the binary tree.
    ///
    /// If the value is already in the tree, it is not added again.
    pub fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    /// Returns the number of values in the binary tree.
    pub fn len(&self) -> usize {
        self.root.len()
    }

    /// Returns true if the binary tree contains a value.
    pub fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }
}

impl<T: Ord> Subtree<T> {
    /// Creates an empty subtree.
    fn new() -> Self {
        Self(None)
    }

    /// Inserts a value into the subtree.
    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => {
                self.0 = Some(Box::new(Node::new(value)));
            }
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => {}
                Ordering::Greater => n.right.insert(value),
            },
        }
    }

    /// Returns the number of values in the subtree.
    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }

    /// Returns true if the subtree contains a value.
    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }
}

impl<T: Ord> Node<T> {
    /// Creates a new node with no children.
    fn new(value: T) -> Self {
        Self {
            value: value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(6);
    assert_eq!(tree.len(), 1);
    tree.insert(7);
    assert!(tree.has(&7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

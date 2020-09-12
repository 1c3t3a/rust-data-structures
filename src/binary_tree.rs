//! A safe binary tree.
//!
//! The `binary tree` allows inserting, removing and is naturally sorted.
//!
//! NOTE: This was written for a learning purpose.
use super::stack::Stack;
use std::cmp::Ordering;

/// A binary tree build from Nodes. This struct represents a binary tree
/// holding a root node.
pub struct BinaryTree<T> {
    head: Link<T>,
}

/// A Link between Nodes.
type Link<T> = Option<Box<Node<T>>>;

/// A Node in a binary tree which holds a reference to the left and right Nodes as well as a value.
#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

/// The Iterator for a binary tree, containing a stack with all nodes that should be visited.
/// Instances are created by [`BinaryTree::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, T: 'a> {
    visited: Box<Stack<&'a Box<Node<T>>>>,
}

impl<T: Eq + std::cmp::Ord> BinaryTree<T> {
    /// Creates a new and empty `BinaryTree`.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::<()>::new();
    /// assert!(binary_tree.is_empty());
    ///```
    pub fn new() -> Self {
        BinaryTree { head: None }
    }

    /// Inserts a new element into the tree. The tree alway keeps an ordered structure by
    /// making sure that the left child node is always "bigger" than the right child node.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(1);
    /// binary_tree.insert(2);
    ///
    /// assert!(binary_tree.find(1));
    /// assert!(binary_tree.find(2));
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(value)));
            return true;
        }
        self.head.as_mut().unwrap().insert(value)
    }
    
    /// Checks if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Searches for an element in the tree and returns whether the tree contains the element
    /// or not. As the tree is always ordered, this happens relatively fast. (As long as it's not)
    /// super unordered.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(1);
    ///
    /// assert!(binary_tree.find(1));
    /// ```
    pub fn find(&self, value: T) -> bool {
        match self.head.as_ref() {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Equal => true,
                Ordering::Greater => match node.right.as_ref() {
                    Some(node) => node.find(value),
                    None => false,
                },
                Ordering::Less => match node.left.as_ref() {
                    Some(node) => node.find(value),
                    None => false,
                },
            },
            None => false,
        }
    }

    /// Traverses the tree inorder. That means it goes through the tree and recursively
    /// visites the leftmost child, the root and then the rightmost child.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(10);
    /// binary_tree.insert(8);
    /// binary_tree.insert(11);
    /// binary_tree.insert(9);
    /// binary_tree.insert(12);
    ///
    /// assert_eq!(binary_tree.inorder(), vec![&8, &9, &10, &11, &12]);
    /// ```
    pub fn inorder(&self) -> Vec<&T> {
        match &self.head {
            Some(node) => node.inorder(),
            None => Vec::new(),
        }
    }

    /// Traverses the tree preorder. That means it goes through the tree and recursively
    /// visites the root, the leftmost child, and then the rightmost child.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(10);
    /// binary_tree.insert(8);
    /// binary_tree.insert(11);
    /// binary_tree.insert(9);
    /// binary_tree.insert(12);
    ///
    /// assert_eq!(binary_tree.preorder(), vec![&10, &8, &9, &11, &12]);
    /// ```
    pub fn preorder(&self) -> Vec<&T> {
        match &self.head {
            Some(node) => node.preorder(),
            None => Vec::new(),
        }
    }

    /// Traverses the tree postorder. That means it goes through the tree and recursively
    /// visites the the leftmost child, the righmost child and then the root node.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(10);
    /// binary_tree.insert(8);
    /// binary_tree.insert(11);
    /// binary_tree.insert(9);
    /// binary_tree.insert(12);
    ///
    /// assert_eq!(binary_tree.preorder(), vec![&10, &8, &9, &11, &12]);
    /// ```
    pub fn postorder(&self) -> Vec<&T> {
        match &self.head {
            Some(node) => node.postorder(),
            None => Vec::new(),
        }
    }

    /// Returns an `Iterator` over the elements of a tree. First the root node is returned,
    /// than an ordering from the lowest to the highest element.
    /// # Example
    /// ```
    /// use data_structure_with_colin::binary_tree::BinaryTree;
    /// let mut binary_tree = BinaryTree::new();
    /// binary_tree.insert(10);
    /// binary_tree.insert(8);
    /// binary_tree.insert(11);
    ///
    /// for elem in binary_tree.iter() {
    ///     println!("{}", elem);   
    /// }
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        let mut visited = Box::new(Stack::new());
        visited.push(self.head.as_ref().unwrap());
        Iter { visited: visited }
    }
}

impl<T: Eq + std::cmp::Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match &mut self.left {
                Some(node) => node.insert(value),
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                    return true;
                }
            },
            Ordering::Equal => return false,
            Ordering::Greater => match &mut self.right {
                Some(node) => node.insert(value),
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                    return true;
                }
            },
        }
    }

    fn find(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Greater => match self.right.as_ref() {
                Some(node) => node.find(value),
                None => false,
            },
            Ordering::Less => match self.left.as_ref() {
                Some(node) => node.find(value),
                None => false,
            },
        }
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut result = vec![];
        match self.left.as_ref() {
            Some(node) => {
                let mut left_vec = node.inorder();
                result.append(left_vec.as_mut());
            }
            None => (),
        }
        result.push(&self.value);
        match self.right.as_ref() {
            Some(node) => {
                let mut right_vec = node.inorder();
                result.append(right_vec.as_mut());
            }
            None => (),
        }
        result
    }

    pub fn postorder(&self) -> Vec<&T> {
        let mut result = vec![];
        match self.left.as_ref() {
            Some(node) => {
                let mut vec_left = node.postorder();
                result.append(vec_left.as_mut());
            }
            None => (),
        }
        match self.right.as_ref() {
            Some(node) => {
                let mut right_vec = node.postorder();
                result.append(right_vec.as_mut());
            }
            None => (),
        }
        result.push(&self.value);
        result
    }

    pub fn preorder(&self) -> Vec<&T> {
        let mut result = vec![];
        result.push(&self.value);
        match self.left.as_ref() {
            Some(node) => {
                let mut vec_left = node.preorder();
                result.append(vec_left.as_mut());
            }
            None => (),
        }
        match self.right.as_ref() {
            Some(node) => {
                let mut right_vec = node.preorder();
                result.append(right_vec.as_mut());
            }
            None => (),
        }
        result
    }
}

impl<'a, T: std::cmp::Eq> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.visited.is_empty() {
            return None;
        }

        let node = self.visited.pop();
        match (
            node.unwrap().as_ref().left.as_ref(),
            node.unwrap().as_ref().right.as_ref(),
        ) {
            (None, None) => (),
            (leaf @ Some(_), None) | (None, leaf @ Some(_)) => self.visited.push(&leaf.unwrap()),
            (Some(left), Some(right)) => {
                self.visited.push(right);
                self.visited.push(left);
            }
        }

        return Some(&node.unwrap().as_ref().value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sut = BinaryTree::new();
        assert!(sut.insert(42));
        assert!(sut.insert(41));
        assert!(sut.insert(43));
    }

    #[test]
    fn test_traversal() {
        let mut sut = BinaryTree::new();
        sut.insert(10);
        sut.insert(8);
        sut.insert(11);
        sut.insert(9);
        sut.insert(12);

        assert_eq!(sut.inorder(), vec![&8, &9, &10, &11, &12]);
        assert_eq!(sut.preorder(), vec![&10, &8, &9, &11, &12]);
        assert_eq!(sut.postorder(), vec![&9, &8, &12, &11, &10]);
    }

    #[test]
    fn test_find() {
        let mut sut = BinaryTree::new();
        sut.insert(10);
        sut.insert(8);
        sut.insert(11);
        sut.insert(9);
        sut.insert(12);

        assert!(sut.find(10));
        assert!(sut.find(8));
        assert!(sut.find(11));
        assert!(sut.find(9));
        assert!(sut.find(12));
    }

    #[test]
    fn test_iter_count() {
        let mut sut = BinaryTree::new();
        sut.insert(10);
        sut.insert(8);
        sut.insert(11);
        sut.insert(9);
        sut.insert(12);

        assert_eq!(sut.iter().count(), 5);
    }

    #[test]
    fn test_iter_elementwise() {
        let mut sut = BinaryTree::new();
        sut.insert(10);
        sut.insert(8);
        sut.insert(11);
        sut.insert(9);
        sut.insert(12);

        let mut iter = sut.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&12));
    }
}

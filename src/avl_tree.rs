use std::{cmp::Ordering, iter::FromIterator, cmp::max};

#[derive(Debug, PartialEq, Clone)]
struct AVLNode<'a, T: Ord> {
    value: T, 
    left: AVLTree<T>,
    right: AVLTree<T>,
    height: usize,
    parent_node: Option<&'a mut AVLNode<'a, T>>
}

type AVLTree<T> = Option<Box<AVLNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AVLTreeSet<T: Ord> {
    root: AVLTree<T>,
}

impl<T: Ord> AVLTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) -> bool {
        let mut current = &mut self.root;

        while let Some(current_node) = current {
            match current_node.value.cmp(&value) {
                Ordering::Less => current = &mut current_node.right,
                Ordering::Equal => return false,
                Ordering::Greater => current = &mut current_node.left,
            }
        }

            *current = Some(Box::new(AVLNode {
                value,
                left: None,
                right: None,
                height: 0
            }));
        true
    }   

}

impl<'a, T: 'a + Ord> AVLNode<T> {
    pub fn balance_factor(&self) -> i8 {
        (self.left_height() - self.right_height()) as i8
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height())
    }

    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height())
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height())
    }

    fn height(&self) -> usize {
        1 + max (
            self.left.as_ref().map_or(0, |node| node.height()),
            self.right.as_ref().map_or(0, |node| node.height())
        )
    }
}

impl<T: Ord> FromIterator<T> for AVLTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
    }
}

/// Iterator

impl<'a, T: 'a + Ord> AVLTreeSet<T> {
    fn iter(&'a self) -> AVLTreeSetIter<'a, T> {
        AVLTreeSetIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
struct AVLTreeSetIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AVLNode<T>>,
    current_tree: &'a AVLTree<T>
}

impl<'a, T: 'a + Ord> Iterator for AVLTreeSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None, 
                    Some(prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some(&prev_node.value);
                    }
                }
                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }
                    
                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;
                        return Some(&current_node.value);
                    }

                    self.current_tree = &None;

                    return Some(&current_node.value);
                }
            }
        }
    }
}

/// Iterator

/// Tests
#[cfg(test)]
mod test {
    use super::*;
    use itertools::{all, equal};
    use quickcheck::TestResult;
    #[test]
    fn test_insert() {
        let mut set = AVLTreeSet::new();

        assert!(set.insert(1));   // Insert new value
        assert!(!set.insert(1));  // Should not insert existing value

        assert!(set.insert(2));   // Insert another new value
        assert_eq!(               // Checking the tree structure
            set.root,
            Some(Box::new(AVLNode {
                value: 1,
                left: None,
                right: Some(Box::new(AVLNode {
                    value: 2,
                    left: None,
                    right: None
                })),
            }))
        );
    }

    #[test]
    fn small_iter_test() {
        let mut set = AVLTreeSet::new();

        for i in (1..4 as usize).rev() {
            set.insert(i);
        }

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    use std::collections::BTreeSet;
    
    #[quickcheck]
    // 1. Create a random list
    fn iterator_parity(mut xs: Vec<usize>) -> bool {
        // 2. Create an empty AVL tree and BTree
        // 3. For each element in the list, insert it to both tree
        let avl_set = xs.iter().cloned().collect::<AVLTreeSet<_>>();
        let btree_set = xs.iter().cloned().collect::<BTreeSet<_>>();

        // 4. Both AVL and BTree iterator should be equal item per item
        equal(avl_set.iter(), btree_set.iter())
    }
}
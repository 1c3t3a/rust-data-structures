extern crate rand;

use crate::avl_tree::tree::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem::replace;
use std::{cmp::Ordering, iter::FromIterator};

#[derive(Debug, PartialEq, Clone)]
struct AVLTreeSet<T: Ord> {
    root: AVLTree<T>,
}

impl<'a, T: 'a + Ord> Default for AVLTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AVLTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<*mut AVLNode<T>>::new();
        let mut current = &mut self.root;

        while let Some(current_node) = current {
            prev_ptrs.push(&mut **current_node);

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
            height: 1,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        let mut prev_ptrs = Vec::<*mut AVLNode<T>>::new();
        let mut current_tree = &mut self.root;
        let mut target_value = None;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(&value) {
                Ordering::Less => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal => {
                    target_value = Some(&mut **current_node);
                    break;
                }
                Ordering::Greater => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.left;
                }
            }
        }

        // Target does not exist in tree
        if target_value.is_none() {
            return None;
        }

        let target_node = target_value.unwrap();

        // 3 Cases: No children, left child, right child
        let taken = if target_node.left.is_none() || target_node.right.is_none() {
            // Left child
            if let Some(left_node) = target_node.left.take() {
                replace(target_node, *left_node).value
            }
            // Right child
            else if let Some(right_node) = target_node.right.take() {
                replace(target_node, *right_node).value
            }
            // No children
            else if let Some(prev_ptr) = prev_ptrs.pop() {
                let prev_node = unsafe { &mut *prev_ptr };

                // Determine which node to take
                let inner_value = if let Some(left_node) = prev_node.left.as_ref() {
                    // Target was left node from parent
                    if left_node.value == target_node.value {
                        prev_node.left.take().unwrap().value
                    }
                    // Target was right node from parent
                    else {
                        prev_node.right.take().unwrap().value
                    }
                }
                // Target was right all along, since left is None
                else {
                    prev_node.right.take().unwrap().value
                };

                prev_node.update_height();
                prev_node.rebalance();

                inner_value
            }
            // Fourth boring case: Tree is root
            else {
                self.root.take().unwrap().value
            }
        }
        // Find Inorder-Successor
        else {
            AVLTreeSet::find_inorder_succesor(target_node)
        };

        // Update for every touched Node
        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        Some(taken)
    }

    fn find_inorder_succesor(target_node: &mut AVLNode<T>) -> T {
        let right_tree = &mut target_node.right;

        // Left tree of right is None, take first right
        if right_tree.as_ref().unwrap().left.is_none() {
            let mut right_node = right_tree.take().unwrap();

            let inner_value = replace(&mut target_node.value, right_node.value);
            replace(&mut target_node.right, right_node.right.take());

            target_node.update_height();
            target_node.rebalance();

            inner_value
        }
        // Take leftest(^^) left node
        else {
            let mut next_tree = right_tree;
            let mut left_ptrs = Vec::<*mut AVLNode<T>>::new();

            // iterate to leftest
            while let Some(next_left_node) = next_tree {
                if next_left_node.left.is_some() {
                    left_ptrs.push(&mut **next_left_node);
                }
                next_tree = &mut next_left_node.left;
            }

            let parent_leftest_node = unsafe { &mut *left_ptrs.pop().unwrap() };

            let mut leftest_node = parent_leftest_node.left.take().unwrap();

            // Taken node is now filled with leftest value
            let inner_value = replace(&mut target_node.value, leftest_node.value);

            // Leftest node is now the right child of former leftest,
            // because leftest has no left child and if right child is none, then thats the end of this tree
            replace(&mut parent_leftest_node.left, leftest_node.right.take());

            // Start at the bottom with updating
            parent_leftest_node.update_height();
            parent_leftest_node.rebalance();

            // Up to the children of target
            // Rev because into iter starts at the first inserted item, we need the last inserted first
            for node_ptr in left_ptrs.into_iter().rev() {
                let node = unsafe { &mut *node_ptr };
                node.update_height();
                node.rebalance();
            }

            // At last of course target node to update
            target_node.update_height();
            target_node.rebalance();

            inner_value
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.take(value).is_some()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.get(value).is_some()
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let mut current_tree = &self.root;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(&value) {
                Ordering::Less => {
                    current_tree = &current_node.right;
                }
                Ordering::Equal => {
                    return Some(&current_node.as_ref().value);
                }
                Ordering::Greater => {
                    current_tree = &current_node.left;
                }
            }
        }
        None
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
    fn iter(&'a self) -> AVLTreeSetNodeIter<'a, T> {
        AVLTreeSetNodeIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
struct AVLTreeSetNodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AVLNode<T>>,
    current_tree: &'a AVLTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AVLTreeSetNodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some(prev_node);
                    }
                },
                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;
                        return Some(current_node);
                    }

                    self.current_tree = &None;

                    return Some(current_node);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_avl_insert_and_remove_basic() {
        let mut tree = AVLTreeSet::new();
        tree.insert(50);
        tree.insert(70);
        tree.insert(90);
        assert_eq!(tree.insert(90), false);
        assert_eq!(tree.contains(&50), true);
        assert_eq!(tree.contains(&70), true);
        assert_eq!(tree.contains(&90), true);
        tree.remove(&70);
        assert_eq!(tree.contains(&70), false);
        assert_eq!(tree.get(&50).unwrap(), &50);
        assert_eq!(tree.root.unwrap().value, 90);
    }

    #[test]
    fn test_insert_randomly() {
        let avl = (1..50 as u8).collect::<AVLTreeSet<_>>();
        let btree = (1..50 as u8).collect::<BTreeSet<_>>();

        for it in avl.iter().zip(btree.iter()) {
            let (a, b) = it;
            assert_eq!(&a.value, b)
        }
    }

    #[test]
    fn test_delete_somehow_randomly() {
        let mut avl = (1..100 as u16).collect::<AVLTreeSet<_>>();
        let mut btree = (1..100 as u16).collect::<BTreeSet<_>>();

        avl.remove(&45);
        avl.remove(&12);
        avl.remove(&36);
        avl.remove(&73);
        avl.remove(&75);
        avl.remove(&80);

        btree.remove(&45);
        btree.remove(&12);
        btree.remove(&36);
        btree.remove(&73);
        btree.remove(&75);
        btree.remove(&80);

        assert_eq!(avl.contains(&90), true);
        assert_eq!(avl.get(&90).unwrap(), &90);

        for it in avl.iter().zip(btree.iter()) {
            let (a, b) = it;
            assert_eq!(&a.value, b)
        }
    }

    #[test]
    fn test_insert() {
        let mut avl = AVLTreeSet::new();
        let mut btree = BTreeSet::new();

        avl.insert(10);
        avl.insert(9);
        avl.insert(4);

        btree.insert(10);
        btree.insert(9);
        btree.insert(4);

        for it in avl.iter().zip(btree.iter()) {
            let (a, b) = it;
            assert_eq!(&a.value, b)
        }
    }

    #[test]
    fn truly_random_insert() {
        let mut vec: Vec<u32> = (0..10000).collect();
        vec.shuffle(&mut thread_rng());
        let mut avl = vec.iter().collect::<AVLTreeSet<_>>();
        let mut btree = vec.iter().collect::<BTreeSet<_>>();

        for it in avl.iter().zip(btree.iter()) {
            let (a, b) = it;
            assert_eq!(&a.value, b)
        }
    }

    #[test]
    fn random_remove() {
        let mut vec: Vec<u32> = (0..100000).collect();
        vec.shuffle(&mut thread_rng());
        let mut avl = vec.iter().collect::<AVLTreeSet<_>>();
        let mut btree = vec.iter().collect::<BTreeSet<_>>();

        let mut remove: Vec<u32> = (0..10000).collect();
        remove.shuffle(&mut thread_rng());
        for item in remove.iter() {
            avl.remove(&item);
            btree.remove(&item);
        }

        for it in avl.iter().zip(btree.iter()) {
            let (a, b) = it;
            assert_eq!(&a.value, b)
        }
    }
}

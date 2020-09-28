use crate::avl_tree::tree::*;
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
            height: 1
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
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
    current_tree: &'a AVLTree<T>
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
                }
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

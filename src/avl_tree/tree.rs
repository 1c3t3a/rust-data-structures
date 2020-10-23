use std::cmp::max;
use std::mem::{replace, swap};

#[derive(Debug, PartialEq, Clone)]
pub struct AVLNode<T: Ord> {
    pub value: T,
    pub left: AVLTree<T>,
    pub right: AVLTree<T>,
    pub height: usize,
}

pub type AVLTree<T> = Option<Box<AVLNode<T>>>;

impl<'a, T: 'a + Ord> AVLNode<T> {
    // Overflow precautions
    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height())
    }

    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    fn rotate_right(&mut self) {
        if self.left.is_none() {
            return;
        }

        let new_center = self.left.as_mut().unwrap();
        let new_left = new_center.left.take();
        let left_of_new_right = new_center.right.take();

        let mut new_right = replace(&mut self.left, new_left);
        swap(&mut self.value, &mut new_right.as_mut().unwrap().value);
        let right_tree = self.right.take();

        let new_right_node = new_right.as_mut().unwrap();
        new_right_node.left = left_of_new_right;
        new_right_node.right = right_tree;
        self.right = new_right;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();
    }

    fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }

        let new_center = self.right.as_mut().unwrap();
        let new_right = new_center.right.take();
        let right_of_new_left = new_center.left.take();

        let mut new_left = replace(&mut self.right, new_right);
        swap(&mut self.value, &mut new_left.as_mut().unwrap().value);
        let left_tree = self.left.take();

        let new_left_node = new_left.as_mut().unwrap();
        new_left_node.left = left_tree;
        new_left_node.right = right_of_new_left;
        self.left = new_left;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();
    }

    pub fn rebalance(&mut self) {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();
            }
            2 => {
                let left_node = self.left.as_mut().unwrap();

                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();
            }
            _ => return,
        }
    }
}

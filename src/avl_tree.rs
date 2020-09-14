use std::cmp::Ordering;

type AVLTree<T> = Option<Box<AVLNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AVLNode<T: Ord> {
    value: T, 
    left: AVLTree<T>,
    right: AVLTree<T>,
}

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
            }));
        true
    }   
}

#[cfg(test)]
mod test {
    use super::*;
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
}
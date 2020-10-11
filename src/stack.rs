//! A safe stack.
//!
//! The `Stack` allows inserting, removing and iterating it's elements.
//!
//! NOTE: This was written for a learning purpose.

use super::linked_list::LinkedList;
use std::iter::FromIterator;

/// A Stack build from Nodes. This struct represents a Stack with a head node and a size.
pub struct Stack<T> {
    first: Link<T>,
    size: i32,
}

/// A link between Nodes.
type Link<T> = Option<Box<Node<T>>>;

/// A node in a stack which holds some value of Type T.
/// It also saves the next node in the stack.
struct Node<T> {
    next: Link<T>,
    value: T,
}

/// An Iter struct for iterating over the stacks elements.
/// Instances are created by [`Stack::iter()`].
pub struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
}

/// An owning Iterator of the stacks elements.
/// Instances are created by [`Stack::into_iter()`]. See its
/// documentation for more.
pub struct IntoIter<T: Eq> {
    stack: Stack<T>,
}

impl<T: Eq> Stack<T> {
    /// Creates a new empty stack.
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::<()>::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn new() -> Self {
        Stack {
            first: None,
            size: 0,
        }
    }

    /// Inserts into a stack. Remember that a stack works in the 'last in first out' principle.
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn push(&mut self, elem: T) {
        if self.is_empty() {
            self.first = Some(Box::new(Node::new(elem)));
            self.size += 1;
        } else {
            let mut new_node = Box::new(Node::new(elem));
            new_node.next = self.first.take();
            self.first = Some(new_node);
            self.size += 1;
        }
    }

    /// Returns the size (depth) of the stack. It iterates over all elements to do so and is therefore not super fast.
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Checks if a stack contains a specific element. It iterates over the elements until it finds the searched one
    /// or the end is reached.
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert!(!stack.contains(2));
    /// assert!(stack.contains(1));
    /// ```
    pub fn contains(&self, value: T) -> bool {
        return if self.is_empty() {
            false
        } else if self.first.as_ref().unwrap().value == value {
            true
        } else {
            self.first.as_ref().unwrap().contains(value)
        };
    }

    /// Pops the first element of a stack. Remember that the stacks first element is always the last one that got inserted (LIFO-Principle).
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(2);
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// assert!(!stack.is_empty());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            let mut old_head = self.first.take();
            self.first = old_head.as_mut().unwrap().next.take();
            self.size -= 1;
            Some(old_head.unwrap().value)
        }
    }

    /// Checks if the current stack is empty or not.
    /// ```rust
    /// use data_structure_with_colin::stack::Stack;
    /// let mut stack = Stack::new();
    /// assert!(stack.is_empty());
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns a non-owning iterator for the stack which iterates over the element in the LIFO way.
    pub fn iter(&self) -> Iter<T> {
        Iter { head: &self.first }
    }
}

impl<T: Eq> Default for Stack<T> {
    /// Creates an empty `LinkedList<T>`.
    #[inline]
    fn default() -> Self {
        Stack::new()
    }
}

impl<T: Eq> From<LinkedList<T>> for Stack<T> {
    fn from(list: LinkedList<T>) -> Self {
        list.into_iter().collect()
    }
}

impl<T: Eq> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Stack::new();
        for elem in iter {
            result.push(elem);
        }
        result
    }
}

impl<T: Eq> Node<T> {
    #[inline]
    fn new(value: T) -> Self {
        Node { next: None, value }
    }

    #[inline]
    fn contains(&self, value: T) -> bool {
        return if self.value == value {
            true
        } else {
            match &self.next {
                Some(node) => node.contains(value),
                None => false,
            }
        };
    }
}

impl<T: Eq> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator over the lists values.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// Returns the next element of a stack.
    fn next(&mut self) -> Option<Self::Item> {
        self.head.as_ref().map(|node| {
            self.head = &node.next;
            &node.value
        })
    }
}

impl<'a, T: Eq> Iterator for IntoIter<T> {
    type Item = T;

    /// Returns the next element of a IntoIter.
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

macro_rules! stack {
    () => {
        Stack::new();
    };
    ($($elem:expr),+) => {{
        let mut res = Stack::new();
        $(res.push($elem);)+
        res
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_stack() {
        let sut = Stack::<()>::new();
        assert!(sut.is_empty());
    }

    #[test]
    fn test_push() {
        let mut sut = Stack::new();
        sut.push(1);
        assert!(!sut.is_empty());
    }

    #[test]
    fn test_push_1_2_3() {
        let mut sut = Stack::new();
        sut.push(1);
        sut.push(2);
        sut.push(3);
        assert!(!sut.is_empty());
        assert_eq!(sut.size(), 3);
    }

    #[test]
    fn test_pop() {
        let mut sut = Stack::new();
        assert_eq!(sut.pop(), None);
        sut.push(1);
        assert_eq!(sut.pop(), Some(1));
        sut.push(2);
        sut.push(3);
        sut.push(4);
        sut.push(5);
        assert_eq!(sut.pop(), Some(5));
        assert_eq!(sut.size(), 3);
    }

    #[test]
    fn test_contains() {
        let mut sut = Stack::new();
        assert_eq!(sut.contains(42), false);
        sut.push(42);
        assert_eq!(sut.contains(42), true);
        sut.push(43);
        sut.push(44);
        sut.push(45);
        sut.push(46);
        assert_eq!(sut.contains(43), true);
        assert_eq!(sut.contains(44), true);
        assert_eq!(sut.contains(45), true);
        assert_eq!(sut.contains(46), true);
        assert_eq!(sut.contains(47), false);
    }

    #[test]
    fn test_iter_count() {
        let sut = stack![1, 2, 3, 4];
        assert_eq!(sut.iter().count(), 4);
    }

    #[test]
    fn test_iter_loop() {
        let sut = stack![1, 2, 3, 4];
        let mut iter_sut = sut.iter();
        assert_eq!(iter_sut.next(), Some(&4));
        assert_eq!(iter_sut.next(), Some(&3));
        assert_eq!(iter_sut.next(), Some(&2));
        assert_eq!(iter_sut.next(), Some(&1));
    }

    #[test]
    fn test_into_iter() {
        let sut = stack![1, 2, 3, 4];
        let mut iter_sut = sut.into_iter();
        assert_eq!(iter_sut.next(), Some(4));
        assert_eq!(iter_sut.next(), Some(3));
        assert_eq!(iter_sut.next(), Some(2));
        assert_eq!(iter_sut.next(), Some(1));
    }

    #[test]
    fn test_size() {
        let mut sut = stack![1, 2, 3, 4];
        assert_eq!(sut.size(), 4);
        sut.pop();
        assert_eq!(sut.size(), 3);
    }

    #[test]
    fn test_default() {
        let sut: Stack<()> = Default::default();
        assert!(sut.is_empty());
        assert_eq!(sut.size(), 0);
    }

    #[test]
    fn test_from() {
        let mut list = LinkedList::new();
        list.insert(12);
        list.insert(13);
        list.insert(14);
        let mut sut = Stack::from(list);

        assert!(sut.contains(12));
        assert!(sut.contains(13));
        assert!(sut.contains(14));
        assert_eq!(sut.pop(), Some(14));
    }
}

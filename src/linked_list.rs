//! A safe linked list.
//!
//! The `LinkedList` allows inserting, removing and iterating it's elements.
//!
//! NOTE: This was written for a learning purpose.

/// A linked list build from Nodes. This struct represents a linked list
/// with a head and it's length.
pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

/// A Link between Nodes.
type Link<T> = Option<Box<Node<T>>>;

/// A Node in a linked list which holds a reference to the next Node as well as a value.
#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    next: Link<T>,
    value: T,
}

/// The Iterator for a linked list, containing the head as well as the size of the list.
/// Instances are created by [`LinkedList::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
    len: usize,
}

/// An owning Iteraror the elements of a linked list.
/// Instances are created by [`LinkedList::into_iter()`]. See its
/// documentation for more.
pub struct IntoIter<T: Eq> {
    list: LinkedList<T>,
}

impl<T> LinkedList<T>
where
    T: Eq,
{
    /// Creates a new and empty `LinkedList`.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::<()>::new();
    /// assert!(linked_list.is_empty());
    ///```
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    /// Creates a `LinkedList` from a `Vec`.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let v = vec![1, 2, 3];
    /// let linked_list = LinkedList::from_vec(v);
    ///
    /// assert!(linked_list.contains(1));
    /// assert!(linked_list.contains(2));
    /// assert!(linked_list.contains(3));
    ///```
    pub fn from_vec(list: Vec<T>) -> Self {
        let mut result = Self::new();
        for elem in list {
            result.insert(elem);
        }
        result
    }

    /// Checks if the list is empty.
    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Appends a new element to the list.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.insert(1);
    /// linked_list.insert(2);
    ///
    /// assert!(linked_list.contains(1));
    /// assert!(linked_list.contains(2));
    /// ```
    pub fn insert(&mut self, val: T) -> bool {
        match &mut self.head {
            Some(first) => {
                self.len += 1;
                first.insert(val)
            }
            None => {
                self.head = Some(Box::new(Node::new(val)));
                self.len += 1;
                true
            }
        }
    }

    /// Checks if a `LinkedList` contains a given element.
    pub fn contains(&self, val: T) -> bool {
        match &self.head {
            Some(first) => first.contains(val),
            None => false,
        }
    }

    /// Removes an element at the given index from the list.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.insert(1);
    /// linked_list.insert(2);
    ///
    /// assert!(linked_list.contains(1));
    /// assert!(linked_list.contains(2));
    ///
    /// linked_list.remove(0);
    ///
    /// assert!(!linked_list.contains(1));
    /// ```
    pub fn remove(&mut self, index: usize) -> bool {
        if index >= self.len {
            false
        } else if index == 0 {
            let mut old_head = self.head.take().unwrap();
            if let Some(new) = old_head.next.take() {
                self.head = Some(new);
            }
            self.len -= 1;
            true
        } else {
            match &mut self.head {
                Some(val) => {
                    self.len -= 1;
                    val.remove(index, 0)
                }
                None => false,
            }
        }
    }

    /// Removes the head and returns it as an Option.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.insert(1);
    /// linked_list.insert(2);
    ///
    /// assert_eq!(linked_list.pop_front(), Some(1))
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let mut old_head = self.head.take().unwrap();
            let res = Some(old_head.value);
            if let Some(next) = old_head.next.take() {
                self.head = Some(next);
                self.len -= 1;
            }
            res
        }
    }

    /// Returns an `Iterator` over the elements of a list.
    /// # Example
    /// ```
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.insert(1);
    /// linked_list.insert(2);
    /// for elem in linked_list.iter() {
    ///     println!("{}", elem);   
    /// }
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            head: &self.head,
            len: self.len,
        }
    }
}

impl<T: Eq> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator over the lists values.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// Returns the next element of a list iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.head.as_ref().map(|head| {
            self.len -= 1;
            self.head = &head.next;
            &head.value
        })
    }

    /// Returns the length of an iterator.
    fn count(self) -> usize {
        self.len
    }
}

impl<'a, T: Eq> Iterator for IntoIter<T> {
    type Item = T;

    /// Returns the next element of a IntoIter.
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> Node<T>
where
    T: Eq,
{
    fn new(value: T) -> Self {
        Node { next: None, value }
    }

    fn insert(&mut self, val: T) -> bool {
        match &mut self.next {
            Some(iter) => iter.insert(val),
            None => {
                self.next = Some(Box::new(Node::new(val)));
                true
            }
        }
    }

    fn contains(&self, val: T) -> bool {
        if val == self.value {
            true
        } else {
            match &self.next {
                Some(iter) => iter.contains(val),
                None => false,
            }
        }
    }

    fn remove(&mut self, index: usize, mut cur: usize) -> bool {
        if cur + 1 == index {
            let mut garbage = self.next.take().unwrap();
            match garbage.next.take() {
                None => true,
                Some(new_link) => {
                    self.next = Some(new_link);
                    true
                }
            }
        } else {
            cur += 1;
            self.next.as_mut().unwrap().remove(index, cur)
        }
    }
}

/// Macro for creating a list with given elements. Works like the Vec![] Macro.
/// # Example
/// ```rust
/// use data_structure_with_colin::linked_list::LinkedList;
/// let linked_list = list![1, 2, 3];
///
/// assert!(linked_list.contains(1));
/// assert!(linked_list.contains(2));
/// assert!(linked_list.contains(3));
/// ```
macro_rules! list {
    () => {
        LinkedList::new();
    };
    ($elem:expr) => {{
        let mut res = LinkedList::new();
        res.insert($elem);
        res
    }};
    ($($elem:expr),+) => {{
        let mut res = LinkedList::new();
        $(res.insert($elem);)+
        res
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let sut = LinkedList::<bool>::new();
        assert_eq!(sut.head, None);
        assert_eq!(sut.is_empty(), true);
    }

    #[test]
    fn test_insert() {
        let mut sut = LinkedList::<i32>::new();
        assert_eq!(&sut.insert(1), &true);
        assert_eq!(sut.head.unwrap().value, (1 as i32));
    }

    #[test]
    fn test_multiple_insert() {
        let mut sut = LinkedList::new();
        sut.insert(3453);
        sut.insert(342);
        for i in 0..10 {
            sut.insert(i);
        }

        for i in 0..10 {
            assert_eq!(sut.contains(i), true);
        }
        assert_eq!(sut.contains(342), true);
        assert_eq!(sut.contains(3453), true);
    }

    #[test]
    fn test_empty_contains() {
        let sut = LinkedList::<i32>::new();
        assert_eq!(sut.contains(42), false);
    }

    #[test]
    fn test_contains() {
        let mut sut = LinkedList::<i32>::new();
        assert_eq!(sut.contains(42), false);
        sut.insert(42);
        assert_eq!(sut.contains(42), true);
    }

    #[test]
    fn test_to_list() {
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from_vec(vector);
        assert_eq!(sut.head.unwrap().value, 1);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from_vec(vector);
        assert_eq!(sut.head.unwrap().next.unwrap().value, 2);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from_vec(vector);
        assert_eq!(sut.head.unwrap().next.unwrap().next.unwrap().value, 3);
    }

    #[test]
    fn test_macro() {
        let sut: LinkedList<u32> = list![];
        assert_eq!(sut.head, None);
        let sut = list![2];
        assert_eq!(sut.head.unwrap().value, 2);
        let sut = list![1, 2, 3];
        assert_eq!(sut.contains(1), true);
        assert_eq!(sut.contains(2), true);
        assert_eq!(sut.contains(3), true);
    }

    #[test]
    fn test_remove_simple() {
        let mut sut: LinkedList<u32> = list![];
        assert_ne!(true, sut.remove(0));
        sut.insert(45);
        assert!(sut.contains(45));
        sut.remove(0);
        assert!(!sut.contains(45))
    }

    #[test]
    fn test_remove_more() {
        let mut sut: LinkedList<u32> = list![];
        sut.insert(45);
        sut.insert(56);
        sut.insert(234);
        sut.insert(4345);
        sut.insert(3532);
        sut.insert(43234);

        assert_eq!(sut.len, 6);
        sut.remove(5);
        assert!(!sut.contains(43234));
        assert_eq!(sut.len, 5);

        sut.remove(2);
        assert!(!sut.contains(234));
        assert_eq!(sut.len, 4);
    }

    #[test]
    fn test_remove_head() {
        let mut sut: LinkedList<u32> = list![];
        sut.insert(45);
        sut.insert(56);
        sut.insert(234);
        sut.insert(4345);
        sut.insert(3532);
        sut.insert(43234);

        assert_eq!(sut.len, 6);
        assert!(sut.contains(45));
        sut.remove(0);
        assert!(!sut.contains(45));
        let val = sut.head.unwrap().value;
        assert_eq!(val, 56);
        assert_eq!(sut.len, 5);
        println!("{}", val);
    }

    #[test]
    fn test_pop_front() {
        let mut sut = LinkedList::new();
        sut.insert(1);
        sut.insert(2);

        assert_eq!(sut.pop_front(), Some(1))
    }

    #[test]
    fn test_iter_count() {
        let sut = list![1, 2, 3, 4, 5];
        assert_eq!(sut.iter().count(), 5);
    }

    #[test]
    fn test_iter_loop() {
        let sut = list![1, 2, 3, 4, 5];
        let mut iter_sut = sut.iter();
        assert_eq!(iter_sut.next(), Some(&1));
        assert_eq!(iter_sut.next(), Some(&2));
        assert_eq!(iter_sut.next(), Some(&3));
        assert_eq!(iter_sut.next(), Some(&4));
        assert_eq!(iter_sut.next(), Some(&5));
    }

    #[test]
    fn test_into_iter() {
        let sut = list![1, 2, 3, 4, 5];
        let mut iter_sut = sut.into_iter();
        assert_eq!(iter_sut.next(), Some(1));
        assert_eq!(iter_sut.next(), Some(2));
        assert_eq!(iter_sut.next(), Some(3));
        assert_eq!(iter_sut.next(), Some(4));
        assert_eq!(iter_sut.next(), Some(5));
    }
}

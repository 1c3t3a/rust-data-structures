//! A safe linked list.
//!
//! The `LinkedList` allows inserting, removing and iterating it's elements.
//!
//! NOTE: This was written for a learning purpose.

use std::convert::From;

/// A linked list build from Nodes. This struct represents a linked list
/// with a head and it's length.
#[derive(Debug, Eq, PartialEq)]
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
    T: Eq + Ord,
{
    /// Creates a new and empty `LinkedList`.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::<()>::new();
    /// assert!(linked_list.is_empty());
    ///```
    #[inline]
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends a new element to the list.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.append(1);
    /// linked_list.append(2);
    ///
    /// assert!(linked_list.contains(1));
    /// assert!(linked_list.contains(2));
    /// ```
    pub fn append(&mut self, val: T) -> bool {
        match &mut self.head {
            Some(first) => {
                self.len += 1;
                first.append(val)
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
    /// linked_list.append(1);
    /// linked_list.append(2);
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

    pub fn is_sorted(&self) -> bool {
        match &self.head {
            Some(head) => head.is_sorted(),
            None => false,
        }
    }

    /// Why Merge-Sort?
    ///
    /// Quick sort works well for sorting in-place.
    /// In particular, most of the operations can be defined in terms of swapping pairs of elements
    /// in an array. To do that, however, you normally "walk" through the array with two pointers
    /// (or indexes, etc.) One starts at the beginning of the array and the other at the end.
    /// Both then work their way toward the middle (and you're done with a particular partition step
    /// when they meet). That's expensive with files, because files are oriented primarily toward reading
    /// in one direction, from beginning to end. Starting from the end and seeking backwards is usually
    /// relatively expensive.

    /// At least in its simplest incarnation, merge sort is pretty much the opposite.
    /// The easy way to implement it only requires looking through the data in one direction,
    /// but involves breaking the data into two separate pieces, sorting the pieces,
    /// then merging them back together.

    /// With a linked list, it's easy to take (for example) alternating elements in one linked list,
    /// and manipulate the links to create two linked lists from those same elements instead.
    /// With an array, rearranging elements so alternating elements go into separate arrays is easy
    /// if you're willing to create a copy as big as the original data, but otherwise rather more
    /// non-trivial.

    /// Likewise, merging with arrays is easy if you merge elements from the source arrays
    /// into a new array with the data in order -- but to do it in place without creating a whole
    /// new copy of the data is a whole different story. With a linked list, merging elements together
    /// from two source lists into a single target list is trivial -- again, you just manipulate links,
    /// without copying elements.

    /// As for using Quicksort to produce the sorted runs for an external merge sort,
    /// it does work, but it's (decidedly) sub-optimal as a rule. To optimize a merge-sort,
    /// you normally want to maximize the lengths of each sorted "run" as you produce it.
    /// If you simply read in the data that will fit in memory, Quicksort it and write it out,
    /// each run will be restricted to (a little less than) the size of the available memory.

    /// You can do quite a bit better than that as a rule though.
    /// You start by reading in a block of data, but instead of using a Quicksort on it, you build a heap.
    /// Then, as you write each item out from the heap into the sorted "run" file, you read another item
    /// in from your input file. If it's larger than the item you just wrote to disk, you insert it into
    /// your existing heap, and repeat.

    /// Items that are smaller (i.e., belong before items that have already been written) you keep
    /// separate, and build into a second heap. When (and only when) your first heap is empty,
    /// and the second heap has taken over all the memory, you quit writing items to the existing "run" file,
    /// and start on a new one.

    /// Exactly how effective this will be depends on the initial order of the data.
    /// In the worst case (input sorted in inverse order) it does no good at all. In the best case
    /// (input already sorted) it lets you "sort" the data in a single run through the input.
    /// In an average case (input in random order) it lets you approximately double the length of
    /// each sorted run, which will typically improve speed by around 20-25%
    /// (though the percentage varies depending on how much larger your data is than the available memory).
    #[inline]
    pub fn sort(&mut self) {
        if self.head.is_none() {
            return;
        }

        let (mut front, mut back) = self.split();

        if front.len > 1 {
            front.sort();
        }

        if back.len > 1 {
            back.sort();
        }

        self.head = Some(Box::new(Node::from(LinkedList::merge(
            &mut front, &mut back,
        ))));
    }

    #[inline]
    fn split(&mut self) -> (LinkedList<T>, LinkedList<T>) {
        let back = self
            .head
            .as_mut()
            .unwrap()
            .get_back(self.len / 2, 0)
            .unwrap();
        let front = self.head.take().unwrap();

        (LinkedList::from(*front), LinkedList::from(*back))
    }

    #[inline]
    fn merge(front: &mut LinkedList<T>, back: &mut LinkedList<T>) -> Self {
        let mut result: Node<T>;

        if front.head.is_none() {
            return LinkedList::from(*back.head.take().unwrap());
        } else if back.head.is_none() {
            return LinkedList::from(*front.head.take().unwrap());
        }

        if front.head.as_ref().unwrap().value <= back.head.as_ref().unwrap().value {
            result = Node::new(front.pop_front().unwrap());
        } else {
            result = Node::new(back.pop_front().unwrap());
        }

        result.set_next(Node::from(LinkedList::merge(front, back)));

        LinkedList::from(result)
    }

    /// Removes the head and returns it as an Option.
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.append(1);
    /// linked_list.append(2);
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
            }
            self.len -= 1;
            res
        }
    }

    /// Returns an `Iterator` over the elements of a list.
    /// # Example
    /// ```
    /// use data_structure_with_colin::linked_list::LinkedList;
    /// let mut linked_list = LinkedList::new();
    /// linked_list.append(1);
    /// linked_list.append(2);
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

impl<T: Eq + Ord> IntoIterator for LinkedList<T> {
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

impl<'a, T: Eq + Ord> Iterator for IntoIter<T> {
    type Item = T;

    /// Returns the next element of a IntoIter.
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> Node<T>
where
    T: Eq + Ord,
{
    #[inline]
    fn set_next(&mut self, new: Node<T>) {
        self.next = Some(Box::new(new));
    }

    #[inline]
    fn get_back(&mut self, index: usize, mut cur: usize) -> Option<Box<Node<T>>> {
        if cur + 1 == index {
            self.next.take()
        } else {
            cur += 1;
            self.next.as_mut().unwrap().get_back(index, cur)
        }
    }

    fn is_sorted(&self) -> bool {
        let look = self;
        match &look.next {
            Some(val) => {
                if look.value <= val.value {
                    return val.is_sorted();
                } else {
                    return false;
                }
            }
            None => true,
        }
    }

    fn new(value: T) -> Self
    where
        T: Ord,
    {
        Node { next: None, value }
    }

    #[inline]
    fn append(&mut self, val: T) -> bool {
        match &mut self.next {
            Some(iter) => iter.append(val),
            None => {
                self.set_next(Node::new(val));
                true
            }
        }
    }

    #[inline]
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

    #[inline]
    fn remove(&mut self, index: usize, mut cur: usize) -> bool {
        if cur + 1 == index {
            let mut garbage = self.next.take().unwrap();
            match garbage.next.take() {
                None => true,
                Some(new_link) => {
                    self.set_next(*new_link);
                    true
                }
            }
        } else {
            cur += 1;
            self.next.as_mut().unwrap().remove(index, cur)
        }
    }

    fn get_length(&self) -> usize {
        let mut count = 1;
        let mut walk = Some(self);

        while walk.unwrap().next.is_some() {
            walk = walk.unwrap().next.as_deref();
            count += 1;
        }

        count
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
        res.append($elem);
        res
    }};
    ($($elem:expr),+) => {{
        let mut res = LinkedList::new();
        $(res.append($elem);)+
        res
    }};
}

/// Creates a `LinkedList` from a `Vec`.
/// # Example
/// ```rust
/// use data_structure_with_colin::linked_list::LinkedList;
/// let v = vec![1, 2, 3];
/// let linked_list = LinkedList::from(v);
///
/// assert!(linked_list.contains(1));
/// assert!(linked_list.contains(2));
/// assert!(linked_list.contains(3));
///```
impl<T> From<Vec<T>> for LinkedList<T>
where
    T: Eq + Ord,
{
    fn from(list: Vec<T>) -> Self {
        let mut result = list![];
        for elem in list {
            result.append(elem);
        }
        result
    }
}

impl<T> From<LinkedList<T>> for Node<T>
where
    T: Eq + Ord,
{
    fn from(list: LinkedList<T>) -> Self {
        *list.head.unwrap()
    }
}

impl<T> From<Node<T>> for LinkedList<T>
where
    T: Eq + Ord,
{
    fn from(node: Node<T>) -> Self {
        let length = node.get_length();
        LinkedList {
            head: Some(Box::new(node)),
            len: length,
        }
    }
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
    fn test_append() {
        let mut sut = LinkedList::<i32>::new();
        assert_eq!(&sut.append(1), &true);
        assert_eq!(sut.head.unwrap().value, (1 as i32));
    }

    #[test]
    fn test_multiple_append() {
        let mut sut = LinkedList::new();
        sut.append(3453);
        sut.append(342);
        for i in 0..10 {
            sut.append(i);
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
        sut.append(42);
        assert_eq!(sut.contains(42), true);
    }

    #[test]
    fn test_to_list() {
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from(vector);
        assert_eq!(sut.head.unwrap().value, 1);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from(vector);
        assert_eq!(sut.head.unwrap().next.unwrap().value, 2);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from(vector);
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
        sut.append(45);
        assert!(sut.contains(45));
        sut.remove(0);
        assert!(!sut.contains(45))
    }

    #[test]
    fn test_remove_more() {
        let mut sut: LinkedList<u32> = list![];
        sut.append(45);
        sut.append(56);
        sut.append(234);
        sut.append(4345);
        sut.append(3532);
        sut.append(43234);

        assert_eq!(sut.len, 6);
        sut.remove(5);
        assert!(!sut.contains(43234));
        assert_eq!(sut.len, 5);

        sut.remove(2);
        assert!(!sut.contains(234));
        assert_eq!(sut.len, 4);
    }

    #[test]
    fn test_is_sorted() {
        let mut sut = list![];
        sut.append(4);
        sut.append(3);
        sut.append(5);

        assert!(!sut.is_sorted());

        sut.remove(0);

        assert!(sut.is_sorted());
    }

    #[test]
    fn test_remove_head() {
        let mut sut: LinkedList<u32> = list![];
        sut.append(45);
        sut.append(56);
        sut.append(234);
        sut.append(4345);
        sut.append(3532);
        sut.append(43234);

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
    fn test_get_length() {
        let mut sut: LinkedList<u32> = list![1, 2, 4, 5, 6];

        let head = sut.head.take().unwrap();

        let test = LinkedList::from(*head);

        assert_eq!(test.len, 5)
    }

    #[test]
    fn test_pop_front() {
        let mut sut = LinkedList::new();
        sut.append(1);
        sut.append(2);

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

    #[test]
    fn test_sort() {
        let mut sut = list![5, 4, 3, 2, 1];
        assert!(!sut.is_sorted());
        sut.sort();
        assert!(sut.is_sorted());

        let mut iter_sut = sut.iter();
        assert_eq!(iter_sut.next(), Some(&1));
        assert_eq!(iter_sut.next(), Some(&2));
        assert_eq!(iter_sut.next(), Some(&3));
        assert_eq!(iter_sut.next(), Some(&4));
        assert_eq!(iter_sut.next(), Some(&5));
    }

    #[test]
    fn test_sort_advanced() {
        let mut sut = list![1938, 234, 239842, 28, 32, 2, 4, 2382, 1093482, 23, 34, 89, 2];
        assert!(!sut.is_sorted());
        sut.sort();
        assert!(sut.is_sorted());
    }
}

use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    next: Link<T>,
    value: T,
}

pub struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
    len: usize,
}

pub struct IntoIter<T: Eq> {
    list: LinkedList<T>,
}

impl<T> LinkedList<T>
where
    T: Eq + Ord,
{
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }

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

    pub fn contains(&self, val: T) -> bool {
        match &self.head {
            Some(first) => first.contains(val),
            None => false,
        }
    }

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
            Some(head) => return head.is_sorted(),
            None => false,
        }
    }

    pub fn sort(&mut self) {
        if self.head == None {
            return;
        }

        //TODO: add length check

        let (mut front, mut back) = self.split();

        front.sort();
        back.sort();

        self.head = Some(Box::new(Node::from(LinkedList::merge(&mut front, &mut back))));
    }

    fn split(&mut self) -> (LinkedList<T>, LinkedList<T>) {
        let back = self.head.as_mut().unwrap().get_back(self.len / 2, 0).unwrap();
        let front = self.head.take().unwrap();

        (LinkedList::from(*front), LinkedList::from(*back))
    }    

    fn merge(front: &mut LinkedList<T>, back: &mut LinkedList<T>) -> Self {
        let mut result: Node<T>;
        
        if front.head.is_none() {
            return LinkedList::from(*back.head.take().unwrap());
        }
        else if back.head.is_none() {
            return LinkedList::from(*front.head.take().unwrap());
        }

        if front.head.as_ref().unwrap().value <= back.head.as_ref().unwrap().value {
            result = Node::new(front.pop_front().unwrap());
            result.set_next( Node::from(LinkedList::merge(front, back)));
        }
        else {
            result = Node::new(back.pop_front().unwrap());

            result.set_next( Node::from(LinkedList::merge(front, back)));
        }
        
        LinkedList::from(result)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let mut old_head = self.head.take().unwrap();
            let res = Some(old_head.value);
            if let Some(next) = old_head.next.take() {
                self.head = Some(next);
            }
            res
        }
    }

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

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
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

    fn count(self) -> usize {
        self.len
    }
}

impl<'a, T: Eq + Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> Node<T>
where
    T: Eq + Ord,
{
    fn set_next(&mut self, new: Node<T>) {
        self.next = Some(Box::new(new));
    }

    fn get_back(&mut self, index: usize, mut cur: usize) -> Option<Box<Node<T>>>{
        if cur + 1 == index {
            self.next.take()
        }
        else {
            cur += 1;
            self.next.as_mut().unwrap().get_back(index, cur)
        }
    }

    fn is_sorted(&self) -> bool {
        let look = self;
        match &look.next {
            Some(val) => {
                if look.value <= val.value {
                    return val.is_sorted()
                }
                else {
                    return false;
                }
            },
            None => return true,
        }
    }

    fn new(value: T) -> Self where T: Ord{
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

#[macro_export]
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

impl<T> From<Vec<T>> for LinkedList<T> where T: Eq + Ord{
    fn from (list: Vec<T>) -> Self {
        let mut result = list![];
        for elem in list {
            result.insert(elem);
        }
        result
    }
}

impl<T> From<LinkedList<T>> for Node<T> where T: Eq + Ord {
    fn from(list: LinkedList<T>) -> Self {
        *list.head.unwrap()
    }
}

impl<T> From<Node<T>> for LinkedList<T> where T: Eq + Ord {
    fn from(node: Node<T>) -> Self {
        let length = node.get_length();
        LinkedList { head: Some(Box::new(node)), len: length }
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
    fn test_is_sorted() {

        let mut sut = list![];
        sut.insert(4);
        sut.insert(3);
        sut.insert(5);

        assert!(!sut.is_sorted());

        sut.remove(0);

        assert!(sut.is_sorted());
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
    fn test_get_length() {
        let mut sut: LinkedList<u32> = list![1,2,4,5,6];

        let head = sut.head.take().unwrap();

        let test = LinkedList::from(*head);

        assert_eq!(test.len, 5)
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
        let mut sut = list![5,4,3,2,1];
        assert!(!sut.is_sorted());
        sut.sort();
        assert!(sut.is_sorted());
    }
}

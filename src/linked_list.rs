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
            return false;
        } else if index == 0 {
            let mut old_head = self.head.take().unwrap();
            if let Some(new) = old_head.next.take() {
                self.head = Some(new);
            }
        }
        match &mut self.head {
            Some(val) => {
                self.len -= 1;
                return val.remove(index, 0);
            }
            None => false,
        }
    }

    pub fn is_sorted(&self) -> bool {
        match &self.head {
            Some(head) => return head.is_sorted(),
            None => false,
        }
    }

    pub fn sort(&mut self) {

    }

    fn split(&mut self) -> (LinkedList<T>, LinkedList<T>) {
        (LinkedList::new(), LinkedList::new())
    }    

    fn merge(front: &mut LinkedList<T>, back: &mut LinkedList<T>) -> Self {
        LinkedList::new()
    }
}

impl<T> Node<T>
where
    T: Eq + Ord,
{
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
            return self.next.as_mut().unwrap().remove(index, cur);
        }
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
        let mut sut: LinkedList<u32> = list![];
        sut.insert(45);
        sut.insert(56);
        sut.insert(234);
        sut.insert(4345);
        sut.insert(3532);
        sut.insert(43234);

        assert!(!sut.is_sorted());

        sut.remove(4);

        assert!(sut.is_sorted());
    }
}

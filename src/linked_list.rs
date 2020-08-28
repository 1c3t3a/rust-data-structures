use std::mem;
#[derive(Debug, Eq, PartialEq)]
pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    next: Link<T>,
    value: T,
}

impl <T> LinkedList<T> where T: Eq {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, val: T) -> bool{
        match &mut self.head {
            Some(first) => first.insert(val),
            None => {
                self.head = Some(Box::new(Node::new(val)));
                true
            },
        }
    }

    pub fn contains(&self, val: T) -> bool {
        match &self.head {
            Some(first) => first.contains(val),
            None => false,
        }
    }

    pub fn remove(&mut self, val: T) -> bool {
        match &mut self.head {
            Some(first) => {
                if first.value == val {
                    mem::replace(first, self.head.unwrap().next.unwrap());
                    true
                } else {
                    first.remove(val)
                }
            },
            None => false,
        }
    }
}

impl <T> Node<T> where T : Eq {
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
        }
        else {
            match &self.next {
                Some(iter) => iter.contains(val),
                None => false,
            }
        }
    }

    fn remove(&mut self, val: T) -> bool {
        match &mut self.next {
            Some(iter) => {
                if iter.value == val {
                    self.next = iter.next;
                    true
                } else {
                    match &mut iter.next {
                        Some(next_n) => next_n.remove(val),
                        None => false
                    }
                }
            },
            None => false,
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
}


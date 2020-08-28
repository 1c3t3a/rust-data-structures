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
    
    pub fn from_list(list: Vec<T>) -> Self {
        let mut result = Self::new();
        for elem in list {
            result.insert(elem);
        }
        result
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

}

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
        let sut = LinkedList::from_list(vector);
        assert_eq!(sut.head.unwrap().value, 1);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from_list(vector);
        assert_eq!(sut.head.unwrap().next.unwrap().value, 2);
        let vector = vec![1, 2, 3];
        let sut = LinkedList::from_list(vector);
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
}


pub struct Stack<T> {
    first: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> Stack<T> {

    fn new() -> Self {
        Stack { first: None }
    }

    fn push(&mut self, elem: T) {
        if self.is_empty() {
            self.first = Some(Box::new(Node::new(elem)));
        } else {
            let mut new_node = Box::new(Node::new(elem));
            new_node.next = self.first;
            self.first.unwrap().next = self.first.take().unwrap().next;
            self.first = Some(new_node);
        }
    }

    fn is_empty(self) -> bool {
        self.first.is_some()
    }

}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { next: None, value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_stack() {
        let sut = Stack::<()>::new();
        assert!(sut.is_empty());
    }

    fn test_push() {
        let sut = Stack::new();
        sut.push(1);
        assert!(!sut.is_empty());
    }

}

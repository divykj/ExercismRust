use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn with_next(mut self, next: Link<T>) -> Self {
        self.next = next;
        self
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node_box = &self.head;
        while let Some(node) = node_box {
            node_box = &node.next;
            len += 1;
        }
        len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut node_box = &self.head;
        let mut ll = Self::new();
        while let Some(node) = node_box {
            node_box = &node.next;
            ll.push(node.data.clone());
        }
        ll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        for _element in _iter {
            ll.push(_element);
        }
        ll
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vector = Vec::<T>::new();
        while let Some(data) = self.pop() {
            vector.push(data);
        }
        vector.reverse();
        vector
    }
}

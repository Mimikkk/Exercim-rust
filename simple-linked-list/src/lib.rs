use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node {data, next}
    }
}


impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self { SimpleLinkedList {head: None} }

    pub fn len(&self) -> usize {
        let mut ln: usize = 0;
        let mut head_node = &self.head;
        while let Some(node) = head_node {
            ln += 1;
            head_node = &node.next;
        }
        ln
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node::new(_element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| { self.head = node.next; node.data })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while let Some(val) = self.pop() { rev_list.push(val); }
        rev_list
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut c = SimpleLinkedList::new();
        for i in _iter { c.push(i); }
        c
    }
}
impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        self.rev().into_iter().collect()
    }
}

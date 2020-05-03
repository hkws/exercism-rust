use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;
        let mut size: usize = 0;
        while node.is_some() {
            if let Some(content) = node {
                size += 1;
                node = &content.next;
            };
        }
        size
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_some()
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(), // takeは取り出したあと元の位置を None にする
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;
        let node = self.head.take().unwrap();
        self.head = node.next;
        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            _ => Some(&self.head.as_ref().unwrap().data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut node_ptr = &self.head;
        let mut new_list = Self::new();
        while node_ptr.is_some() {
            if let Some(content) = node_ptr {
                new_list.push(content.data.clone());
                node_ptr = &content.next;
            }
        }
        new_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut new_list = Self::new();
        for item in _iter.into_iter() {
            new_list.push(item);
        }
        new_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:

// let vec: Vec<_> = simple_linked_list.into_iter().collect();

// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut new_vec = std::vec::Vec::with_capacity(self.len());
        while let Some(content) = self.pop() {
            new_vec.push(content);
        }
        new_vec.reverse();
        new_vec
    }
}

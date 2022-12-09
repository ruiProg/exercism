use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: NextNode<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;
        let mut length = 0;

        while let Some(node_element) = node {
            length += 1;
            node = &node_element.next;
        }
        length
    }

    pub fn push(&mut self, element: T) {
        let mut new_node = Box::new(NodeElement::new(element));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();
        if let Some(ref mut head_element) = head {
            self.head = head_element.next.take();
        }
        head.map(|val| val.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|val| &val.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut current_node = self.head.take();
        self.head = None;

        while let Some(mut node_element) = current_node {
            current_node = node_element.next.take();
            node_element.next = self.head;
            self.head = Some(node_element);
        }
        self
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for val in iter {
            list.push(val);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut linked_list = linked_list.rev();
        let mut vec = Vec::new();

        while let Some(val) = linked_list.pop() {
            vec.push(val);
        }
        vec
    }
}

struct NodeElement<T> {
    data: T,
    next: NextNode<T>,
}

impl<T> NodeElement<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

type NextNode<T> = Option<Box<NodeElement<T>>>;

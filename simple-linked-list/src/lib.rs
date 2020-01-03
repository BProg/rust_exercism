use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node::new(_element));
        let mut last = &mut self.head;
        while last.is_some() && last.as_ref().unwrap().next.is_some() {
            last = &mut last.as_mut().unwrap().next;
        }
        if let Some(last) = &mut last {
            last.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut last: &mut Option<Box<Node<T>>> = &mut self.head;
        let mut last_value: Option<T> = last.as_ref().map(|b| (*b).value);

        for i in 1..self.len {
            last = if let Some(node) = last {
                if i == self.len - 1 {
                    last_value = node.next.as_ref().map(|b| (*b).value);
                    node.next = None
                }
                &mut node.next
            } else {
                last
            }
        }

        if self.len == 1 {
            self.head = None;
        }

        self.len = if self.len == 0 {
            self.len
        } else {
            self.len - 1
        };
        last_value
    }

    pub fn peek(&self) -> Option<&T> {
        let mut last: &Option<Box<Node<T>>> = &self.head;
        let mut last_value: Option<&T> = last.as_ref().map(|b| &(*b).value);

        for i in 1..self.len {
            last = if let Some(node) = last {
                if i == self.len - 1 {
                    last_value = node.next.as_ref().map(|b| &(*b).value);
                }
                &node.next
            } else {
                last
            }
        }

        last_value
    }

    // TODO: remove mut
    pub fn rev(self) -> SimpleLinkedList<T> {
        let v: Vec<T> = self.into();
        v.into_iter().rev().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T>
where
    T: Copy + std::fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Copy + std::fmt::Debug,
{
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        let mut last: &Option<Box<Node<T>>> = &self.head;

        let take_value_and_push_in_vec = |node: &Option<Box<Node<T>>>, vec: &mut Vec<T>| {
            if let Some(node_ptr) = node {
                vec.push(node_ptr.value);
            }
        };

        take_value_and_push_in_vec(last, &mut vec);

        for _i in 1..self.len {
            last = if let Some(node) = last {
                &node.next
            } else {
                last
            };
            take_value_and_push_in_vec(last, &mut vec);
        }

        vec
    }
}

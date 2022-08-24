// following this tutorial
// https://rust-unofficial.github.io/too-many-lists/fourth.html

use std::cell::RefCell;
use std::rc::Rc;

pub struct Deq<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Deq<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                                            // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                                            // total: +2 new_head -- OK!
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 new
                    // not emptying list
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head); // +1 new
                                                // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                                      // total: -2 old, (no new)
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut deq = Deq::new();

        // Check empty list behaves right
        assert_eq!(deq.pop_front(), None);

        // Populate list
        deq.push_front(1);
        deq.push_front(2);
        deq.push_front(3);

        // Check normal removal
        assert_eq!(deq.pop_front(), Some(3));
        assert_eq!(deq.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deq.push_front(4);
        deq.push_front(5);

        // Check normal removal
        assert_eq!(deq.pop_front(), Some(5));
        assert_eq!(deq.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(deq.pop_front(), Some(1));
        assert_eq!(deq.pop_front(), None);
    }
}

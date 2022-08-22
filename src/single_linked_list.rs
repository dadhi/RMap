// following this tutorial
// following this tutorial
// https://rust-unofficial.github.io/too-many-lists/second.html


// ## Stack as a mutable linked list implementation
// Omg, null pointer optimizing enum - basically Empty case is represented as 0 without need for tag,
// because the only other case is always not null, because it contains a non-zero pointer.
struct StackNode<T> {
    elem: T,
    next: Option<Box<StackNode<T>>>,
}

// single field struct is zero-cost abstraction
pub struct Stack<T> {
    head: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(StackNode {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a StackNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut StackNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_into_iter_as_it_is_basically_reverse() {
        let mut s = Stack::<i32>::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);

        let vec = s.into_iter().map(|x| x + 1).collect::<Vec<_>>();

        assert_eq!(vec![5, 4, 3, 2], vec);
    }

    #[test]
    fn test_linked_stack() {
        let mut s = Stack::<i32>::new();
        assert_eq!(None, s.pop());

        s.push(42);
        assert_eq!(Some(42), s.pop());
        assert_eq!(None, s.pop());

        s.push(13);
        s.push(14);
        assert_eq!(Some(14), s.pop());
        assert_eq!(Some(13), s.pop());
        assert_eq!(None, s.pop());

        s.push(42);
        assert_eq!(Some(&42), s.peek());
        assert_eq!(Some(42), s.pop());

        // test peek_mut
        s.push(42);
        let res = s.peek_mut().map(|v| {
            let res = *v; // todo: @improve
            *v = 43;
            res
        });
        assert_eq!(Some(42), res);
        assert_eq!(Some(&43), s.peek());
    }
}

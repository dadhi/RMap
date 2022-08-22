use std::rc::Rc;

pub struct ImList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> ImList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        Self { head: Some(Rc::new(Node {
            elem,
            next: self.head.clone(),
        }))}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Self {
        Self { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend() {
        let list = ImList::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);
        assert_eq!(list.head.as_ref().unwrap().elem, 3);
        assert_eq!(list.tail().head.as_ref().unwrap().elem, 2);
        assert_eq!(list.tail().tail().head.as_ref().unwrap().elem, 1);
    }

    #[test]
    fn test_head_and_tail() {
        let list = ImList::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail().head(), Some(&2));
        assert_eq!(list.tail().tail().head(), Some(&1));
        assert_eq!(list.tail().tail().tail().head(), None);
    }
}
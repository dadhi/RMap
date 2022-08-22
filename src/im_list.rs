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
            elem: elem,
            next: self.head.clone(),
        }))}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true)
    }
}
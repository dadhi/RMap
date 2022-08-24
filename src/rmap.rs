// the original https://gist.github.com/vsraptor/96b60b2758919fa34769c8e4d5d49a9f

#![allow(dead_code)]

#[derive(Debug)]
pub struct RMap<T> {
    hash: i32,
    value: T,
    left: Option<Box<RMap<T>>>,
    right: Option<Box<RMap<T>>>,
}

impl<T: Clone> RMap<T> {
    pub fn new(h: i32, v: T) -> RMap<T> {
        RMap {
            hash: h,
            value: v,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, h: i32, v: T) {
        if self.hash == h {
            return;
        };

        let branch = if h > self.hash {
            &mut self.right
        } else {
            &mut self.left
        };

        match branch {
            Some(branch) => branch.add(h, v),
            None => *branch = Some(Box::new(RMap::new(h, v))),
        }
    }

    pub fn get_value(&self, h: i32) -> Option<T> {
        if h == self.hash {
            Some(self.value.clone()) // todo: @wip avoid clone
        } else if h < self.hash {
            self.left.as_ref()?.get_value(h)
        } else if h > self.hash {
            self.right.as_ref()?.get_value(h)
        } else {
            None
        }
    }
}

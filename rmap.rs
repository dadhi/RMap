// the original https://gist.github.com/vsraptor/96b60b2758919fa34769c8e4d5d49a9f

#![allow(dead_code)]

use std::string::String;

#[derive(Debug)]
struct RMap<T> {
    hash: i32,
    value: T,
    left: Option<Box<RMap<T>>>,
    right: Option<Box<RMap<T>>>,
}

impl<T: Clone> RMap<T> {
    fn new(h: i32, v: T) -> RMap<T> {
        RMap {
            hash: h,
            value: v,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, h:i32, v: T) {
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

    fn get_value(&self, h: i32) -> Option<T> {
        if h == self.hash {
            Some(self.value.clone()) // todo: @wip avoid clone
        }

        else if h < self.hash {
            self.left.as_ref()?.get_value(h)
        } else if h > self.hash {
            self.right.as_ref()?.get_value(h)
        } else {
            None
        }
    }
}

fn main() {
    let mut m1 = RMap::new(5, String::from("foo"));
    m1.add(7, String::from("bar"));
    m1.add(3, String::from("baz"));

    println!("{:#?}", m1);

    let mut m2 = RMap::new(3, String::from("ay"));
    m2.add(5, String::from("all"));
    m2.add(1, String::from("bell"));

    println!("{:#?}", m2);

    let res = m2.get_value(1);
    println!("{:#?}", res);
}

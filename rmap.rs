// the original https://gist.github.com/vsraptor/96b60b2758919fa34769c8e4d5d49a9f

#![allow(dead_code)]

use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::string::String;

#[derive(Debug)]
struct RMap<T> {
    content: T,
    left: Option<Box<RMap<T>>>,
    right: Option<Box<RMap<T>>>,
}

impl<T: PartialEq + PartialOrd> RMap<T> {
    fn new(val: T) -> RMap<T> {
        RMap {
            content: val,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, val: T) {
        if self.content == val {
            return;
        }; //already exists
        let update = if val > self.content {
            &mut self.right
        } else {
            &mut self.left
        };
        match update {
            Some(update) => update.add(val),                     //dig deeper
            None => *update = Some(Box::new(RMap::new(val))), //add a leaf
        }
    }

    fn search(&self, target: T) -> Option<T> {
        if target == self.content {
            Some(target)
        }
        //found
        else if target < self.content {
            self.left.as_ref()?.search(target)
        } else if target > self.content {
            self.right.as_ref()?.search(target)
        } else {
            None
        } //not found
    }
}

fn main() {
    let mut root = RMap::new(5);
    root.add(7);
    root.add(3);

    println!("{:#?}", root);

    let mut s = RMap::new(String::from("angle"));
    s.add(String::from("all"));
    s.add(String::from("bell"));

    println!("{:#?}", s);
    let res = s.search(String::from("bell"));
    println!("{:#?}", res);
}

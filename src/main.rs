#![allow(dead_code)]

pub mod rmap;
mod samples;

use crate::rmap::RMap;

fn main() {
    rule_110();
}

fn rule_110() {
    println!("## RULE-110 premier:");

    const GEN_SIZE: usize = 30;
    for _ in 0..GEN_SIZE {
        print!("-");
    }
    print!("*");
    println!();
}


fn play_with_rmap() {
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


#![allow(unused)]

pub mod rmap;
mod samples;

use crate::rmap::RMap;

fn main() {
    rule_110();
}

fn rule_110() {
    use std::io::{self, Write};

    println!("## RULE-110 premier:");
    const GEN_SIZE: usize = 30;
    const SYMBOLS: [&str; 2] = ["-", "+"];
    const STR: &[u8; 2] = b"-+";

    let mut gen = [0u8; GEN_SIZE];

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for n in 0..GEN_SIZE {
        // usize to str

        stdout.write(&n.to_string().as_bytes()); // @perf remove heap alloc with numtoa crate
        stdout.write(b" ");
        stdout.write(STR);
        //print!("{}", SYMBOLS[gen[0] as usize]);
        let mut pattern = gen[0] << 1 | gen[1];
        for i in 2..GEN_SIZE {
            pattern = (pattern << 1 | gen[i]) & 3; // keep pattern as 3 lower bits, 000, 001, 010, 011, etc.
            gen[i - 1] = (110 >> pattern) & 1; // convert to index in binary representation of 110, and find set bit
        }
        // stdout new line
        stdout.write(b"\n");
    }
    stdout.flush();

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

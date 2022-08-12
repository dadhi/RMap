#![allow(unused)]

pub mod rmap;
mod samples;

use crate::rmap::RMap;

fn main() {
    rule_110();
}

use std::io::{self, Write, Result};

fn rule_110() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // const GEN_SIZE: usize = 30;
    const SYMBOLS: [&[u8; 1]; 2] = [b"-", b"+"];
    const D: u8 = 48;
    const DIGITS: [u8; 10] = [D+0, D+1, D+2, D+3, D+4, D+5, D+6, D+7, D+8, D+9];
    const SPACE: u8 = 32;
    
    stdout.write(b"## RULE-110 premier. 110 in binary representation is 01101110:\n");

    let mut cells: usize = 0;
    cells |= 1 << 31; // init the last cell with 1


    for d1 in 0..3 {
        for d0 in 0..10 {
            
            stdout.write(&[DIGITS[d1], DIGITS[d0], SPACE])?; // todo: @perf reuse the array

            stdout.write(SYMBOLS[cells & 1])?;
            stdout.write(SYMBOLS[(cells >> 1) & 1])?;

            let mut pattern = ((cells & 1) << 1) | ((cells >> 1) & 1);

            for i in 2..32 {
                stdout.write(SYMBOLS[(cells >> i) & 1])?;

                pattern = (pattern << 1 | ((cells >> i) & 1)) & 0b0000_0111; // keep pattern as 3 lower bits, 000, 001, 010, 011, etc.

                // clear i - 1 bit first
                cells &= !(1 << (i - 1));
                // convert to index in binary representation of 110, and set i - 1 bit
                cells |= ((110 >> pattern) & 1) << (i - 1);

            }
            // stdout new line
            stdout.write(b"\n")?;
        }
    }
    stdout.flush()?;
    Ok(())
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

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

    const SYMBOLS: [&[u8; 1]; 2] = [b"-", b"#"];
    const SYMBOLS_01: [&[u8; 2]; 4] = [b"--", b"-#", b"#-", b"##"];
    const DIGITS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    const SPACE: u8 = b' ';
    
    stdout.write(b"## RULE-110 premier. 110 in binary representation is 01101110:\n");

    let mut cells: usize = 1 << 31; // init the last cell with 1

    let mut gen_num: [u8; 3] = [0, 0, SPACE]; // the buffer to store and output the generation number

    for d0 in 0..3 {
        for d1 in 0..10 {
            (gen_num[0], gen_num[1]) = (DIGITS[d0], DIGITS[d1]);
            stdout.write(&gen_num)?;

            let mut pattern = ((cells & 1) << 1) | ((cells >> 1) & 1);
            stdout.write(SYMBOLS_01[pattern])?; // todo: @perf funny that we may just replace it with b"--" for 30 gens, cause it is always will be "--"

            for i in 2..32 {
                stdout.write(SYMBOLS[(cells >> i) & 1])?;

                pattern = (pattern << 1 | ((cells >> i) & 1)) & 0b0000_0111; // keep pattern as 3 lower bits, 000, 001, 010, 011, etc.

                // clear i - 1 bit first
                cells &= !(1 << (i - 1));
                // get pattern's bit in 110 (in its binary form 0b_0110_1110) and set to it the i - 1 bit in cells
                cells |= ((110 >> pattern) & 1) << (i - 1);

            }

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

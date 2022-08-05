pub mod rmap;

use crate::rmap::RMap;

fn fac(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * fac(n - 1)
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// expression calculator
fn calc_from(expr: String, pos: usize, len: usize) -> i32 {
    let mut chars = expr.chars();
    let mut i = pos;
    let mut a: i32 = 0;

    let mut c = chars.nth(i).unwrap();
    while i < len {
        if let Some(d) = c.to_digit(10) {
            a = d as i32;
            i += 1;
            while i < len {
                c = chars.next().unwrap();
                if let Some(d) = c.to_digit(10) {
                    a = a * 10 + d as i32;
                    i += 1;
                } else {
                    break;
                }
            }
        }

        if c == '+' {
            a += calc_from(expr, i + 1, len);
            break;
        }

        // skip not supported operators
        i += 1;
        if let Some(cc) = chars.next() {
            c = cc;
        } else {
            break;
        }
    }
    a
}

fn calc(expr: &str) -> i32 {
    if expr.len() == 0 {
        0
    } else {
        calc_from(expr.to_owned(), 0, expr.len())
    }
}

fn main() {
    println!("fac of 5: {}", fac(5));
    println!("fib of 5: {}", fib(5));
    println!("calc of 1 + 3: {}", calc("1 + 3"));

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

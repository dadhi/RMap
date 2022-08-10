pub mod rmap;

use crate::rmap::RMap;

fn fac(n: i32) -> i128 {
    if n == 0 {
        1
    } else {
        (n as i128) * fac(n - 1)
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
fn insertion_sort(v: &mut [i32]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[test]
fn test_insertion_sort() {
    let mut v = [3, 5, 4, 1, 2];
    insertion_sort(&mut v);
    assert_eq!(v, [1, 2, 3, 4, 5]);
}


fn main() {
    println!("fac of 5: {}", fac(10));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fac() {
        assert_eq!(fac(0), 1);
        assert_eq!(fac(1), 1);
        assert_eq!(fac(2), 2);
        assert_eq!(fac(3), 6);
        assert_eq!(fac(4), 24);
        assert_eq!(fac(5), 120);
        assert_eq!(fac(6), 720);
        assert_eq!(fac(7), 5040);
        assert_eq!(fac(8), 40320);
        assert_eq!(fac(9), 362880);
        assert_eq!(fac(10), 3628800);
        assert_eq!(fac(11), 39916800);
        assert_eq!(fac(12), 479001600);
        assert_eq!(fac(13), 6227020800);
        assert_eq!(fac(14), 87178291200);
        assert_eq!(fac(15), 1307674368000);
        assert_eq!(fac(16), 20922789888000);
        assert_eq!(fac(17), 355687428096000);
        assert_eq!(fac(18), 6402373705728000);
        assert_eq!(fac(19), 121645100408832000);
        assert_eq!(fac(20), 2432902008176640000);
        assert_eq!(fac(21), 51090942171709440000);
        assert_eq!(fac(22), 1124000727777607680000);
        assert_eq!(fac(23), 25852016738884976640000);
        assert_eq!(fac(24), 620448401733239439360000);
        assert_eq!(fac(25), 15511210043330985984000000);
        assert_eq!(fac(26), 403291461126605635584000000);
        assert_eq!(fac(27), 10888869450418352160768000000);
        assert_eq!(fac(28), 304888344611713860501504000000);
    }
}

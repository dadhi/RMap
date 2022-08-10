mod rmap;

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

#[cfg(test)]
fn insertion_sort(v: &mut [i32]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
fn lcs(s1: &str, s2: &str) -> String {
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let mut res = String::new();
    let mut i = s1.len();
    let mut j = s2.len();
    while i > 0 && j > 0 {
        if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
            res.insert(0, s1.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    res
}

// todo: implement Rule 110 in rust
// fn rule_110() {
// }

#[cfg(test)]
fn fib_memo(n: i32) -> i32 {
    if (n == 0) || (n == 1) {
        n
    } else {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2usize..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}

#[cfg(test)]
fn fib_memo_optimized(n: i32) -> i32 {
    if (n == 0) || (n == 1) {
        n
    } else {
        let mut n_minus_2 = 0;
        let mut n_minus_1 = 1;
        let mut res = 0;
        for _ in 2usize..=n as usize {
            res = n_minus_1 + n_minus_2;
            n_minus_2 = n_minus_1;
            n_minus_1 = res;
        }
        res
    }
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fac() {
        assert_eq!(fac(0), 1);
        assert_eq!(fac(1), 1);
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

    #[test]
    fn test_fib_memo_optimized() {
        assert_eq!(fib_memo_optimized(0), 0);
        assert_eq!(fib_memo_optimized(1), 1);
        assert_eq!(fib_memo_optimized(2), 1);
        assert_eq!(fib_memo_optimized(3), 2);
        assert_eq!(fib_memo_optimized(4), 3);
    }

    #[test]
    fn test_fib_memo() {
        assert_eq!(fib_memo(0), 0);
        assert_eq!(fib_memo(1), 1);
        assert_eq!(fib_memo(2), 1);
        assert_eq!(fib_memo(3), 2);
        assert_eq!(fib_memo(4), 3);
    }

    #[test]
    fn test_lcs() {
        assert_eq!(lcs("abcde", "ace"), "ace");
        assert_eq!(lcs("abc", "abc"), "abc");
        assert_eq!(lcs("abc", "def"), "");
        assert_eq!(lcs("abc", ""), "");
        assert_eq!(lcs("", "abc"), "");
        assert_eq!(lcs("", ""), "");
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = [3, 5, 4, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn manipulate_bits() {
        let mut byte: u8 = 0b0000_0000;

        byte |= 0b0000_1000; // Set a bit
        println!("0b{:08b}", byte);

        byte &= 0b1111_0111; // Unset a bit
        println!("0b{:08b}", byte);

        byte ^= 0b0000_1000; // Toggle a bit
        println!("0b{:08b}", byte);

        byte = !byte; // Reverse bits
        println!("0b{:08b}", byte);

        byte <<= 1; // shift left one bit
        println!("0b{:08b}", byte);

        byte >>= 1; // shift right one bit
        println!("0b{:08b}", byte);

        byte = byte.rotate_left(1); // rotate left one bit
        println!("0b{:08b}", byte);

        byte = byte.rotate_right(1); // rotate right one bit
        println!("0b{:08b}", byte);
    }

    fn preorder_traverse(root: &RNode) -> Vec<i32> {
        let mut res = vec![];
        do_rec(root, &mut res);
        return res;

        fn do_rec(root: &RNode, res: &mut Vec<i32>) {
            if let Some(ref left) = *root.left {
                do_rec(left, res)
            }
            res.push(root.val);
            if let Some(ref right) = *root.right {
                do_rec(right, res);
            }
        }
    }

    struct RNode {
        val: i32,
        left: Box<Option<RNode>>,
        right: Box<Option<RNode>>,
    }

    impl RNode {
        fn new(val: i32) -> Self {
            RNode {
                val: val,
                left: Box::new(None),
                right: Box::new(None),
            }
        }
    }

    #[test]
    fn test_preorder_traversal() {
        let mut root = RNode::new(1);
        root.left = Box::new(Some(RNode::new(2)));
        root.right = Box::new(Some(RNode::new(3)));

        assert_eq!(vec![2, 1, 3], preorder_traverse(&root));

        if let Some(ref mut rl) = root.left.as_mut() {
            rl.left = Box::new(Some(RNode::new(4)));
            rl.right = Box::new(Some(RNode::new(5)));
        }

        assert_eq!(vec![4, 2, 5, 1, 3], preorder_traverse(&root));
    }
}

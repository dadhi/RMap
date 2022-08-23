#[cfg(test)]
pub mod tests {
    use std::ops::Deref;


    /// Problem 7: Flatten a nested list structure.
    fn p_flatten_list<T>(list: &Vec<NestedList<T>>) -> Vec<&T> {
        fn flatten_list_rec<'a, T>(list: &'a Vec<NestedList<T>>, res: &mut Vec<&'a T>) {
            for item in list {
                match item {
                    NestedList::Elem(i) => res.push(i),
                    NestedList::List(l) => flatten_list_rec(l, res),
                }
            }
        }
        let mut result = Vec::new();
        flatten_list_rec(list, &mut result);
        result
    }

    #[derive(Debug, PartialEq)]
    enum NestedList<T> {
        Elem(T),
        List(Vec<NestedList<T>>),
    }

    #[test]
    fn p7_test() {
        let list = vec![
            NestedList::Elem(1),
            NestedList::List(vec![
                NestedList::Elem(2),
                NestedList::List(vec![NestedList::Elem(3), NestedList::Elem(4)]),
                NestedList::Elem(5),
            ]),
        ];
        assert_eq!(p_flatten_list(&list), vec![&1, &2, &3, &4, &5]);
    }

    /// Problem 6: Find out whether a list is a palindrome.
    fn p6_is_palindrome<T: PartialEq + Eq>(list: &[T]) -> bool {
        match list {
            [] | [_] => true,
            [a, b] | [a, _, b] if a == b => true,
            [a, mid @ .., b] => a == b && p6_is_palindrome(mid),
        }
    }

    #[test]
    fn test_p6_is_palindrome() {
        assert!(p6_is_palindrome(&[0, 0]));
        assert!(p6_is_palindrome(&[1, 2, 1]));
        assert!(p6_is_palindrome(&[1, 2, 3, 2, 1]));
        assert!(!p6_is_palindrome(&[1, 2, 3, 4, 5]));
    }

    /// Problem 5: Reverse a list
    fn p5_reverse<T>(xs: &mut [T]) {
        match xs {
            [] | [_] => {}
            [a, b] | [a, _, b] => {
                xs.swap(0, xs.len() - 1);
            }
            [_, ..] => {
                let len = xs.len();
                let mid = len / 2;
                for i in 0..mid {
                    xs.swap(i, len - i - 1);
                }
            }
        }
    }

    #[test]
    fn test_p5_reverse() {
        let mut xs = [1, 2, 3];
        p5_reverse(&mut xs);
        assert_eq!(xs, [3, 2, 1]);
    }

    /// Problem 4: Length of a list
    fn p4_length<T>(xs: &[T]) -> usize {
        fn length_rec<T>(xs: &[T], len: usize) -> usize {
            match xs {
                [] => len,
                [_, rest @ ..] => length_rec(rest, len + 1),
            }
        }
        length_rec(xs, 0)
    }

    #[test]
    fn test_p4_length() {
        assert_eq!(p4_length(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(p4_length(EMPTY_I32), 0);
    }

    /// Problem 3: N'th element of a list
    fn p3_nth<T>(xs: &[T], n: usize) -> Option<&T> {
        match xs {
            [] => None,
            [x, ..] if n == 0 => Some(x),
            [_, xs @ ..] => p3_nth(xs, n - 1),
        }
    }

    #[test]
    fn test_p3_nth() {
        let list = [1, 2, 3, 4, 5];
        assert_eq!(p3_nth(&list, 2), Some(&3));
        assert_eq!(p3_nth(&list, 5), None);
        assert_eq!(p3_nth(EMPTY_I32, 0), None);
    }

    /// Problem 2: Last two elements of a list
    fn p2_last_two<T>(xs: &[T]) -> Option<(&T, &T)> {
        match xs {
            [] | [_] => None,
            [pre_last, last] => Some((pre_last, last)),
            [_, rest @ ..] => p2_last_two(rest),
        }
    }

    #[test]
    fn test_p2_last_two() {
        assert_eq!(p2_last_two(&[1, 2, 3, 4]), Some((&3, &4)));
        assert_eq!(p2_last_two(&[1, 4]), Some((&1, &4)));
        assert_eq!(p2_last_two(&[1]), None);
        assert_eq!(p2_last_two(EMPTY_I32), None);
    }

    /// Problem 1: Write a function last : 'a list -> 'a option that returns the last element of a list
    fn p2_last<T>(xs: &[T]) -> Option<&T> {
        match xs {
            [] => None,
            [last] => Some(last),
            [_, rest @ ..] => p2_last(rest),
        }
    }

    #[test]
    fn test_p1_last() {
        assert_eq!(p2_last(&[1, 2, 3, 4]), Some(&4));
        assert_eq!(p2_last(&[1]), Some(&1));
        assert_eq!(p2_last(EMPTY_I32), None);
    }

    const EMPTY_I32: &[i32] = &[];
}

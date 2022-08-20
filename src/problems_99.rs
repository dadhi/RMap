#[cfg(test)]
pub mod tests {
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
        assert_eq!(p4_length(&[] as &[i32]), 0);
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
        assert_eq!(p3_nth(&[] as &[i32], 0), None);
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
        assert_eq!(p2_last_two(&[] as &[i32]), None);
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
        assert_eq!(p2_last(&[] as &[i32]), None);
    }
}
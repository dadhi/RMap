#[cfg(test)]
pub mod tests {
    /// Problem 3: N'th element of a list
    fn p3_nth<T>(list: &[T], n: usize) -> Option<&T> {
        if n < list.len() {
            Some(&list[n])
        } else {
            None
        }
    }

    #[test]
    fn test_p3_nth() {
        let list = [1, 2, 3, 4, 5];
        assert_eq!(p3_nth(&list, 2), Some(&3));
        assert_eq!(p3_nth(&list, 5), None);
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

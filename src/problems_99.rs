#[cfg(test)]
pub mod tests {

    /// Problem 1: Write a function last : 'a list -> 'a option that returns the last element of a list
    fn p_last<T>(xs: &[T]) -> Option<&T> {
        match xs {
            [] => None,
            [last] => Some(last),
            [_, rest @ ..] => p_last(rest),
        }
    }

    /// Problem 2: Last two elements of a list
    fn p_last_two<A>(xs: &[A]) -> Option<(&A, &A)> {
        match xs {
            [] | [_] => None,
            [pre_last, last] => Some((pre_last, last)),
            [_, rest @ ..] => p_last_two(rest),
        }
    }

    #[test]
    fn test_p_last() {
        assert_eq!(p_last(&[1, 2, 3, 4]), Some(&4));
        assert_eq!(p_last(&[1]), Some(&1));
        assert_eq!(p_last(&[] as &[i32]), None);
    }

    #[test]
    fn test_з_last_two() {
        assert_eq!(p_last_two(&[1, 2, 3, 4]), Some((&3, &4)));
        assert_eq!(p_last_two(&[1, 4]), Some((&1, &4)));
        assert_eq!(p_last_two(&[1]), None);
        assert_eq!(p_last_two(&[] as &[i32]), None);
    }
}

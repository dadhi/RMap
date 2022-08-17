#[cfg(test)]
pub mod tests {

    trait Problems<A> {
        /// Problem 1: Write a function last : 'a list -> 'a option that returns the last element of a list
        fn p_last(&self) -> Option<&A>;
    }

    impl<A> Problems<A> for &[A] {
        fn p_last(&self) -> Option<&A> {
            match self {
                [] => None,
                [last] => Some(last),
                [_, tail @ ..] => self.last(),
            }
        }
    }


    #[test]
    fn test_last() {
        assert_eq!([1, 2, 3, 4].as_ref().p_last(), Some(&4));
        assert_eq!([1].as_ref().p_last(), Some(&1));
        let empty_slice: &[i32] = &[];
        assert_eq!(empty_slice.p_last(), None);
    } 
}
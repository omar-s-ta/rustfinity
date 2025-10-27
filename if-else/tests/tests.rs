#[cfg(test)]
mod tests {
    use if_else::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
        assert!(is_even(0));
        assert!(is_even(-2));
        assert!(!is_even(-3));
    }
}

#[cfg(test)]
mod tests {
    use is_prime::*;

    #[test]
    fn test_is_prime_small_numbers() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }

    #[test]
    fn test_is_prime_large_numbers() {
        assert!(is_prime(29));
        assert!(!is_prime(49));
        assert!(is_prime(97));
    }

    #[test]
    fn test_is_prime_even_numbers() {
        assert!(!is_prime(10));
        assert!(!is_prime(12));
        assert!(!is_prime(14));
    }

    #[test]
    fn test_is_prime_odd_numbers() {
        assert!(!is_prime(9));
        assert!(is_prime(11));
        assert!(is_prime(13));
    }
}

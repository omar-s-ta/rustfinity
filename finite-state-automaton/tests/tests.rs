#[cfg(test)]
mod tests {
    use finite_state_automaton::*;

    #[test]
    fn test_recognize_pattern_valid() {
        assert!(recognize_pattern("abbbc"));
        assert!(recognize_pattern("ac"));
        assert!(!recognize_pattern("ab"));
        assert!(recognize_pattern("abbc"));
        assert!(recognize_pattern("abbbbbc"));
    }

    #[test]
    fn test_recognize_pattern_invalid() {
        assert!(!recognize_pattern("abbbd"));
        assert!(!recognize_pattern(""));
        assert!(!recognize_pattern("a"));
        assert!(!recognize_pattern("abbd"));
        assert!(!recognize_pattern("abbbcc"));
    }

    #[test]
    fn test_recognize_pattern_edge_cases() {
        assert!(recognize_pattern("abbbbbc"));
        assert!(!recognize_pattern("a"));
        assert!(recognize_pattern("abc"));
        assert!(recognize_pattern("abbc"));
        assert!(!recognize_pattern("ab"));
    }

    #[test]
    fn test_recognize_pattern_long_input() {
        let long_input_valid = "a".to_owned() + "b".repeat(434).as_str() + "c";
        let long_input_invalid = "a".to_owned() + "b".repeat(333).as_str() + "d";
        assert!(recognize_pattern(&long_input_valid));
        assert!(!recognize_pattern(&long_input_invalid));
        assert!(!recognize_pattern(&(long_input_valid.clone() + "c")));
    }

    #[test]
    fn test_recognize_pattern_random_cases() {
        assert!(recognize_pattern("abbbbbbbbbc"));
        assert!(!recognize_pattern("abbbbbbbbd"));
        assert!(!recognize_pattern("aabbcc"));
        assert!(recognize_pattern("abbbc"));
        assert!(!recognize_pattern("acc"));
    }
}

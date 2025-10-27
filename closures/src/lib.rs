pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    (|a, b| a + b, |a, b| a - b, |a, b| a * b)
}

// Example usage
pub fn main() {
    let (add, subtract, multiply) = create_closures();

    // Example tests
    assert_eq!(add(3, 4), 7); // Expected: 7
    assert_eq!(subtract(10, 4), 6); // Expected: 6
    assert_eq!(multiply(3, 5), 15); // Expected: 15
}

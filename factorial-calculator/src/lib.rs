pub fn factorial(n: u32) -> u128 {
    (1..=n as u128).reduce(|acc, e| acc * e).unwrap_or(1)
}

pub fn is_prime(n: u32) -> bool {
    // rustfinity is using old rust version
    // n >= 2 && !(2..=n.isqrt()).any(|i| n.is_multiple_of(i))
    n >= 2 && !(2..=(n as f64).sqrt() as u32).any(|i| n % i == 0)
}

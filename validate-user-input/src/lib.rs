pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    match ((0..=120).contains(&age), email.contains("@")) {
        (false, _) => Err("Invalid age".into()),
        (_, false) => Err("Invalid email".into()),
        _ => Ok(()),
    }
}

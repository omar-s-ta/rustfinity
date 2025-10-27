pub fn _parse_percentage(input: &str) -> Result<u8, String> {
    if let Ok(number) = input.parse::<u8>() {
        if (0..=100).contains(&number) {
            Ok(number)
        } else {
            Err("Percentage out of range".to_string())
        }
    } else {
        Err("Invalid input".to_string())
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(value) if value <= 100 => Ok(value),
        Ok(_) => Err("Percentage out of range".to_string()),
        Err(_) => Err("Invalid input".to_string()),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}

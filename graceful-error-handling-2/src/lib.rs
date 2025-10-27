use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Invalid input"),
            ParsePercentageError::OutOfRange => write!(f, "Percentage out of range"),
        }
    }
}

impl Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(value) if value <= 100 => Ok(value),
        Ok(_) => Err(ParsePercentageError::OutOfRange),
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}

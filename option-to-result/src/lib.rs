pub fn _get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    let first_element = numbers.first().ok_or("Vector is empty".to_string())?;
    if *first_element < min_value {
        Err("First element is below the minimum allowed value".to_string())
    } else {
        Ok(*first_element)
    }
}

pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    numbers
        .first()
        .ok_or("Vector is empty".to_string())
        .and_then(|&n| {
            if n < min_value {
                Err("First element is below the minimum allowed value".to_string())
            } else {
                Ok(n)
            }
        })
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

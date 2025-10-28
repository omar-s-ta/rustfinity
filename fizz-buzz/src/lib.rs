pub fn fizz_buzz(num: u32) -> String {
    match (num.rem_euclid(3) == 0, num.rem_euclid(5) == 0) {
        (true, true) => "FizzBuzz".into(),
        (true, _) => "Fizz".into(),
        (_, true) => "Buzz".into(),
        _ => num.to_string(),
    }
}

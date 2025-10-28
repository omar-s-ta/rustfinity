pub fn describe_number(n: i32) -> String {
    match (n, n > 0, n % 2 == 0) {
        (0, _, _) => "Zero".into(),
        (_, true, true) => "Positive even".into(),
        (_, true, false) => "Positive odd".into(),
        (_, false, true) => "Negative even".into(),
        _ => "Negative odd".into(),
    }
}

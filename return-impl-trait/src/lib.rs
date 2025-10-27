pub fn filter_starts_with<'a>(
    input: &'a [String],
    keyword: &str,
) -> impl Iterator<Item = &'a String> {
    let prefix = keyword.to_string();
    input.iter().filter(move |arg| arg.starts_with(&prefix))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}

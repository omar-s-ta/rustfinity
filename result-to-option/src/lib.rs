pub fn read_file(file_path: &str) -> Option<String> {
    let contents = std::fs::read_to_string(file_path).ok()?;
    if contents.starts_with("Cannot") {
        None
    } else {
        Some(contents)
    }
}

pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}

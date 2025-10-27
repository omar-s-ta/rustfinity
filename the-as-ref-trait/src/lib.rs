pub fn print_message<T: AsRef<str>>(st: T) {
    println!("{}", st.as_ref())
}

pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}

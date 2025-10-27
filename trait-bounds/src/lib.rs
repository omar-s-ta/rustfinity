use std::fmt::Display;

pub fn _compare_and_display<T>(a: T, b: T) -> T
where
    T: std::fmt::Display + PartialOrd,
{
    if a > b {
        println!("{} {}", a, b);
        a
    } else {
        b
    }
}

pub fn compare_and_display<T: Display + PartialOrd>(a: T, b: T) -> T {
    if a > b {
        println!("{} {}", a, b);
        a
    } else {
        b
    }
}

pub fn main() {
    let greater = compare_and_display(10, 20);
    println!("Greater value: {}", greater);

    let greater = compare_and_display("Apple", "Orange");
    println!("Greater value: {}", greater);
}

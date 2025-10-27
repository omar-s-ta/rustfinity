use std::io::{self, BufRead};

pub fn _sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for num_str in reader.lines() {
        let num = num_str?
            .parse::<i32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
        sum += num;
    }
    Ok(sum)
}

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut sum = 0;
    for line in file.lines() {
        let num = line
            .parse::<i32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
        sum += num;
    }
    Ok(sum)
}

pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

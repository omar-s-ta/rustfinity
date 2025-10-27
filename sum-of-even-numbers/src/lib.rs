pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    (start..=end).filter(|e| e.rem_euclid(2) == 0).sum()
}

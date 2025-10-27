pub fn count_characters(s: &str) -> u32 {
    s.chars().count().try_into().unwrap()
}

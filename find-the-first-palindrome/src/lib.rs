pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let is_palindrome = |n: &i32| {
        let ns = n.to_string();
        ns.chars().eq(ns.chars().rev())
    };
    (start.min(end)..=start.max(end)).find(is_palindrome)
}

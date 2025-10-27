pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i16,
    pub likes: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}

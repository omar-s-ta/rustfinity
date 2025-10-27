pub fn get_database_url() -> String {
    match std::env::var("DATABASE_URL") {
        Ok(url) if url.starts_with("postgresql://") => url,
        Ok(_) => panic!("DATABASE_URL must start with 'postgresql://'"),
        Err(_) => panic!("DATABASE_URL environment variable is not set."),
    }
}

/// Example usage
pub fn main() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    std::env::remove_var("DATABASE_URL"); // Missing variable scenario
    get_database_url();

    std::env::set_var("DATABASE_URL", "mysql://localhost"); // Invalid prefix scenario
    get_database_url();
}

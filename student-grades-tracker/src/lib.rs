use std::collections::HashMap;

pub struct Student {
    pub grades: Vec<u8>,
}

#[derive(Default)]
pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        if self.students.contains_key(name) {
            return;
        }
        self.students
            .insert(name.to_string(), Student { grades: vec![] });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        self.students.get_mut(name).unwrap().grades.push(grade);
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        &self.students.get(name).unwrap().grades
    }
}

pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}

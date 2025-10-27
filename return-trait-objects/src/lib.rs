pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        "Beep boop".to_string()
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => Box::new(Dog {
            name: String::from("name"),
            breed: String::from("breed"),
        }),
        "robot" => Box::new(Robot {
            model: String::from("model"),
            purpose: String::from("purpose"),
        }),
        // Return a Robot instance here
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}

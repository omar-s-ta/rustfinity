#[derive(Clone)]
enum State {
    Start,
    A,
    B,
    C,
    Invalid,
}

pub fn recognize_pattern(input: &str) -> bool {
    let state = input.chars().fold(State::Start, |acc, ch| match (acc, ch) {
        (State::Start, 'a') => State::A,
        (State::A, 'b') => State::B,
        (State::A, 'c') => State::C,
        (State::B, 'b') => State::B,
        (State::B, 'c') => State::C,
        _ => State::Invalid,
    });
    matches!(state, State::C)
}

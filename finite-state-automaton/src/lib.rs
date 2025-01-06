pub fn recognize_pattern(input: &str) -> bool {
    // Implement your finite state automaton here
    let mut fsa = FSA {
        state: State::Start,
    };

    for c in input.chars() {
        match (fsa.state, c) {
            (State::Start, 'a') => fsa.state = State::A,
            (State::A, 'b') => fsa.state = State::B,
            (State::A, 'c') => fsa.state = State::C,
            (State::B, 'b') => fsa.state = State::B,
            (State::B, 'c') => fsa.state = State::C,
            _ => fsa.state = State::Reject,
        }
    }
    if fsa.state == State::C {
        return true;
    }

    false
}

#[derive(PartialEq)]
enum State {
    Start,
    A,
    B,
    C,
    Reject,
}

struct FSA {
    state: State,
}

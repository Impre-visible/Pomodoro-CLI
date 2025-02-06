pub enum State {
    Work,
    Pause,
    Break,
}

impl State {
    pub fn next(&self) -> State {
        match self {
            State::Work => State::Pause,
            State::Pause => State::Break,
            State::Break => State::Work,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            State::Work => "Cycle {}/{}: Work".to_string(),
            State::Pause => "Cycle {}/{}: Take a {} minute break".to_string(),
            State::Break => "Cycle {}/{}: Take a {} minute break".to_string(),
        }
    }
}
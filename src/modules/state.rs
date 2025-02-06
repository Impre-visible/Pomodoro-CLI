#[derive(Clone, Copy)]
pub enum State {
    Work,
    Pause,
    Break,
}

impl State {
    pub fn to_string(&self) -> String {
        match self {
            State::Work => "Cycle {}/{}: Work".to_string(),
            State::Pause => "Cycle {}/{}: Take a pause".to_string(),
            State::Break => "Cycle {}/{}: Take a well-deserved break".to_string(),
        }
    }
}
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
            _ => "Cycle {}/{}: Take a break".to_string(),
        }
    }
}
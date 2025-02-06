#[derive(Clone)]
pub enum TickType {
    NewCycle,
    NewSecond,
}

use super::state::State;

pub struct TickNotifier {
    callbacks: Vec<Box<dyn Fn(TickType, State, u64, u64, u64)>>,
}

impl Copy for TickType {}

impl TickNotifier {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn register(&mut self, callback: Box<dyn Fn(TickType, State, u64, u64, u64)>) {
        self.callbacks.push(callback);
    }

    pub fn notify(&self, tick_type: TickType, state: State, value1: u64, value2: u64, value3: u64) {
        for callback in self.callbacks.iter() {
            callback(tick_type, state, value1, value2, value3);
        }
    }
}
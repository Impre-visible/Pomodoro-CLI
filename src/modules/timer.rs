use super::tick_notifier::{TickNotifier, TickType};
use super::state::State;

pub struct Timer {
    tick_notifier: TickNotifier,
}

impl Timer {
    pub fn new(tick_notifier: TickNotifier) -> Self {
        Self {
            tick_notifier,
        }
    }

    pub fn start(&self, current_cycle: u32, total_cycle: u32, state: State, minute: u64) {      
        self.tick_notifier.notify(TickType::NewCycle, state, u64::from(current_cycle), u64::from(total_cycle), 0);
        
        for remaining in (1..=minute*60).rev() {
            self.tick_notifier.notify(TickType::NewSecond, state, u64::from(current_cycle), u64::from(total_cycle), remaining);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
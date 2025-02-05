use super::display::Display;
use super::notify::Notify;
use super::state::State;

pub struct Timer {
    display: Display,
    notify: Notify,
}

impl Timer {
    pub fn new(display: Display, notify: Notify) -> Self {
        Self { display, notify }
    }

    pub fn start(&self, current_cycle: u32, total_cycle: u32, state: State, minute: u64) {
        for remaining in (1..=minute*60).rev() {
            let message = match state {
                State::Work => format!("Cycle {}/{}: Work", current_cycle, total_cycle),
                State::Pause => format!("Cycle {}/{}: Take a {} minute break", current_cycle, total_cycle, minute),
                State::Break => format!("Cycle {}/{}: Take a {} minute break", current_cycle, total_cycle, minute),
            };
            
            self.notify.notify(message.as_str());
            self.display.display_timer(message.as_str(), remaining);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
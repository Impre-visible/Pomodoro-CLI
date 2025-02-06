use super::display::Display;
use super::notify::Notify;
use super::state::State;

use super::formatter::format_string;

pub struct Timer {
    display: Display,
    notify: Notify,
}

impl Timer {
    pub fn new(display: Display, notify: Notify) -> Self {
        Self { display, notify }
    }

    pub fn start(&self, current_cycle: u32, total_cycle: u32, state: State, minute: u64) {
        let mut message = format_string(state.to_string(), &[current_cycle.to_string(), total_cycle.to_string()]);
        
        self.notify.notify(message.as_str());
        
        for remaining in (1..=minute*60).rev() {
            message = format_string(state.to_string(), &[current_cycle.to_string(), total_cycle.to_string()]);
            self.display.display_timer(message.as_str(), remaining);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
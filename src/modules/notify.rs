use notify_rust::Notification;

use super::state::State;
use super::tick_notifier::TickType;
use super::formatter::format_string;

pub struct Notify {}

impl Notify {    
    pub fn on_tick(tick_type: TickType, state: State, current_cycle: u64, total_cycle: u64, _remaining: u64) {
        if matches!(tick_type, TickType::NewSecond) {
            return;
        }
        let message = &format_string(state.to_string(), &[current_cycle.to_string(), total_cycle.to_string()]);
        Self::notify(message);
    }

    fn notify(message: &str) {
        let _ = Notification::new()
            .summary("Pomodoro")
            .body(message)
            .show();
    }
}
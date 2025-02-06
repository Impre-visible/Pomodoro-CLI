use std::io::Write;

use super::state::State;
use super::tick_notifier::TickType;
use super::formatter::format_string;

pub struct Display {}

impl Display {
    pub fn on_tick(_tick_type: TickType, state: State, current_cycle: u64, total_cycle: u64, remaining: u64) {
        let formatted_time = format!("{:02}:{:02}", remaining / 60, remaining % 60);

        Self::clear_screen();
        Self::display_message(&format_string(state.to_string(), &[current_cycle.to_string(), total_cycle.to_string()]));
        Self::display_message(&format!("Time remaining: {}", formatted_time));
        Self::flush();
    }

    fn display_message(message: &str) {
        println!("{}", message);
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[H");
    }

    fn flush() {
        let _ = std::io::stdout().flush();
    }
}
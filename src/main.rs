mod config;
mod display;

use std::{thread, time::Duration};
use notify_rust::Notification;


use config::Config;
use display::Display;


fn start_pomodoro(config: &Config, display: &Display) {
    for cycle in 1..=config.pomodoro_cycles {
        countdown(display, config.work_duration, &format!("Cycle {}/{}: Work", cycle, config.pomodoro_cycles));
        
        let break_duration = if cycle % config.pomodoro_cycles == 0 {
            config.long_break_duration
        } else {
            config.short_break_duration
        };

        countdown(display, break_duration, &format!("Cycle {}/{}: Take a {} minute break", cycle, config.pomodoro_cycles, break_duration));
    }
}

fn countdown(display: &Display, minutes: u64, message: &str) {
    notify(message);
    let total_seconds = minutes * 60;

    for remaining in (1..=total_seconds).rev() {
        display.display_timer(message, remaining);
        thread::sleep(Duration::from_secs(1));
    }
}

fn notify(message: &str) {
    let _ = Notification::new()
        .summary("Pomodoro")
        .body(message)
        .show();
}

fn main() {
    let config = Config::load();
    let display = Display::new();
    start_pomodoro(&config, &display);
    notify("Pomodoro completed!");
}

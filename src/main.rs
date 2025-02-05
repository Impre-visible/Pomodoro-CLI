use std::{thread, time::Duration, io::Write};
use notify_rust::Notification;

mod config;
use config::Config;


fn start_pomodoro(config: &Config) {
    for cycle in 1..=config.pomodoro_cycles {
        countdown(config.work_duration, &format!("Cycle {}/{}: Work", cycle, config.pomodoro_cycles));
        
        let break_duration = if cycle % config.pomodoro_cycles == 0 {
            config.long_break_duration
        } else {
            config.short_break_duration
        };

        countdown(break_duration, &format!("Cycle {}/{}: Take a {} minute break", cycle, config.pomodoro_cycles, break_duration));
    }
}

fn countdown(minutes: u64, message: &str) {
    notify(message);
    let total_seconds = minutes * 60;

    for remaining in (1..=total_seconds).rev() {
        print_timer(remaining, message);
        thread::sleep(Duration::from_secs(1));
    }
}

fn notify(message: &str) {
    let _ = Notification::new()
        .summary("Pomodoro")
        .body(message)
        .show();
}

fn print_timer(seconds: u64, message: &str) {
    let formatted_time = format!("{:02}:{:02}", seconds / 60, seconds % 60);

    print!("\x1B[2J\x1B[H");
    println!("{}", message);
    println!("Time remaining: {}", formatted_time);

    let _ = std::io::stdout().flush();
}

fn main() {
    let config = Config::load();
    start_pomodoro(&config);
    notify("Pomodoro completed!");
}

use std::{fs, io::Write, thread, time::Duration, path::PathBuf};
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use notify_rust::Notification;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    pomodoro_cycles: u32,
    work_duration: u64,
    short_break_duration: u64,
    long_break_duration: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pomodoro_cycles: 4,
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 20,
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "imprevisible", "pomodoro-cli")
        .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
}

fn load_config() -> Config {
    let config_path = match get_config_path() {
        Some(path) => path,
        None => return Config::default(),
    };

    if config_path.exists() {
        fs::read_to_string(&config_path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_else(Config::default)
    } else {
        save_default_config(&config_path)
    }
}

fn save_default_config(config_path: &PathBuf) -> Config {
    let default_config = Config::default();
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(config_path, toml::to_string(&default_config).unwrap());
    default_config
}

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
    let config = load_config();
    start_pomodoro(&config);
}

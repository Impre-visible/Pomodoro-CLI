use std::{thread, time::Duration, io::Write};
use terminal_size::{Width, Height, terminal_size};
use serde::{Deserialize, Serialize};
use std::fs;
use directories::ProjectDirs;
use notify_rust::Notification;


#[derive(Debug, Deserialize, Serialize)]
struct Config {
    pomodoro_cycles: u32,
    work_duration: u64,
    short_break_duration: u64,
    long_break_duration: u64,
}

fn get_config_path() -> Option<std::path::PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "imprevisible", "pomodoro-cli")?;
    Some(proj_dirs.config_dir().join("config.toml"))
}

fn start_pomodoro(config: Config) {
    let mut cycle: u32 = 1;

    loop {
        countdown(config.work_duration * 60, format!("Cycle {}/{}: Work for {} minutes", cycle, config.pomodoro_cycles, config.work_duration));
        
        if cycle % config.pomodoro_cycles != 0 {
            countdown(config.short_break_duration * 60, format!("Cycle {}/{}: Take a {} minute break", cycle, config.pomodoro_cycles, config.short_break_duration));
        } else {
            countdown(config.long_break_duration * 60, format!("Cycle {}/{}: Take a {} minute break", cycle, config.pomodoro_cycles, config.long_break_duration));
            cycle = 0;
        }

        cycle += 1;
    }
}

fn format_seconds(seconds: u64) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn countdown(seconds: u64, string: String) {
    Notification::new()
        .summary("Pomodoro")
        .body(&string)
        .show()
        .unwrap();
    let size = terminal_size();

    if let Some((Width(_w), Height(_h))) = size {
        for remaining in (1..=seconds).rev() {
            println!("{}", string);
            println!("Time remaining: {}", format_seconds(remaining));

            let size = terminal_size();
            let (_w, h) = if let Some((Width(_w), Height(h))) = size {
                (_w, h)
            } else {
                (0, 0)
            };

            for _ in 0..h-3 {
                println!();
            }

            let _ = std::io::stdout().flush();
            thread::sleep(Duration::from_secs(1));
        }
        println!();
    } else {
        println!("Unable to get terminal size");
    }
}

fn load_config() -> Config {
    let default_config = Config {
        pomodoro_cycles: 4,
        work_duration: 25,
        short_break_duration: 5,
        long_break_duration: 20,
    };

    if let Some(config_path) = get_config_path() {
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path).unwrap_or_default();
            toml::from_str(&config_content).unwrap_or(default_config)
        } else {
            let toml_string = toml::to_string(&default_config).unwrap();
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&config_path, toml_string).unwrap();
            default_config
        }
    } else {
        default_config
    }
}

fn main() {
    let config = load_config();
    start_pomodoro(config);
}
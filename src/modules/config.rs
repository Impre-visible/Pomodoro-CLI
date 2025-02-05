use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub pomodoro_cycles: u32,
    pub work_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
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

impl Config {
    pub fn load() -> Self {
        let project_dirs = ProjectDirs::from("com", "imprevisible", "pomodoro-cli").unwrap();
        let config_path = project_dirs.config_dir().join("config.toml");

        if !config_path.exists() {
            let config = Config::default();
            let toml = toml::to_string(&config).unwrap();
            fs::create_dir_all(project_dirs.config_dir()).unwrap();
            fs::write(&config_path, toml).unwrap();
            return config;
        }

        let toml = fs::read_to_string(&config_path).unwrap();
        toml::from_str(&toml).unwrap()
    }
}
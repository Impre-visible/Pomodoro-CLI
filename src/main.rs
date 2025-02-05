mod config;
mod display;
mod timer;
mod notify;

use config::Config;
use display::Display;
use timer::Timer;
use notify::Notify;


fn start_pomodoro(config: &Config, timer: &Timer) {
    for cycle in 1..=config.pomodoro_cycles {
        timer.start(&format!("Cycle {}/{}: Work", cycle, config.pomodoro_cycles), config.work_duration);
        
        let break_duration = if cycle % config.pomodoro_cycles == 0 {
            config.long_break_duration
        } else {
            config.short_break_duration
        };

        timer.start(&format!("Cycle {}/{}: Take a {} minute break", cycle, config.pomodoro_cycles, break_duration), break_duration);
    }
}

fn main() {
    let config = Config::load();

    let display = Display::new();
    let notify = Notify::new();

    let timer = Timer::new(display, notify);
    
    start_pomodoro(&config,  &timer);
}

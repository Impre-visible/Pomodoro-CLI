mod modules;

use modules::config::Config;
use modules::display::Display;
use modules::timer::Timer;
use modules::notify::Notify;
use modules::state::State;


fn start_pomodoro(config: &Config, timer: &Timer) {
    let mut state: State;
    let mut duration: u64;

    for cycle in 1..=config.pomodoro_cycles {
        state = State::Work;
        duration = config.work_duration;

        timer.start( cycle, config.pomodoro_cycles, state, duration);
        

        if cycle % config.pomodoro_cycles == 0 {
            state = State::Break;
            duration = config.long_break_duration;
        } else {
            state = State::Pause;
            duration = config.short_break_duration;
        };

        timer.start(cycle, config.pomodoro_cycles, state, duration);
    }
}

fn main() {
    let config = Config::load();

    let display = Display::new();
    let notify = Notify::new();

    let timer = Timer::new(display, notify);

    start_pomodoro(&config,  &timer);
}

mod modules;

use modules::config::Config;
use modules::display::Display;
use modules::timer::Timer;
use modules::notify::Notify;
use modules::state::State;
use modules::tick_notifier::TickNotifier;

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


    let mut tick_notifier = TickNotifier::new();

    tick_notifier.register(Box::new(Display::on_tick));
    tick_notifier.register(Box::new(Notify::on_tick));

    let timer = Timer::new(tick_notifier);

    start_pomodoro(&config,  &timer);
}

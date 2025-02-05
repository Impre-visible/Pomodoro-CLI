use std::{thread, time::Duration, io::Write};
use terminal_size::{Width, Height, terminal_size};

const POMODORO_CYCLES: u32 = 4;

const WORK_DURATION: u64 = 25;
const SHORT_BREAK_DURATION: u64 = 5;
const LONG_BREAK_DURATION: u64 = 20;

fn start_pomodoro() {
    let mut cycle: u32 = 1;

    loop {
        countdown(WORK_DURATION * 60, format!("Cycle {}/{}: Work for {} minutes", cycle, POMODORO_CYCLES, WORK_DURATION));
        
        if cycle % POMODORO_CYCLES != 0 {
            countdown(SHORT_BREAK_DURATION * 60, format!("Cycle {}/{}: Take a {} minute break", cycle, POMODORO_CYCLES, SHORT_BREAK_DURATION));
        } else {
            countdown(LONG_BREAK_DURATION * 60, format!("Cycle {}/{}: Take a {} minute break", cycle, POMODORO_CYCLES, LONG_BREAK_DURATION));
            cycle = 0;
        }

        cycle += 1;
    }
}

fn format_seconds(seconds: u64) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn countdown(seconds: u64, string: String) {
    let size = terminal_size();

    if let Some((Width(w), Height(h))) = size {
        for remaining in (1..=seconds).rev() {
            println!("{}", string);
            println!("Time remaining: {}", format_seconds(remaining));

            let size = terminal_size();
            let (w, h) = if let Some((Width(w), Height(h))) = size {
                (w, h)
            } else {
                (0, 0)
            };

            for _ in 0..h-3 {
                println!();
            }

            std::io::stdout().flush();
            thread::sleep(Duration::from_secs(1));
        }
        println!(); // Saute une ligne après le décompte
    } else {
        println!("Unable to get terminal size");
    }
}

fn main() {
    start_pomodoro();
}
use std::{thread, time::Duration, io::Write};
use terminal_size::{Width, Height, terminal_size};



fn start_pomodoro() {
    let mut cycle: u32 = 1;

    loop {
        countdown(1 * 60, format!("Cycle {}/4: Work for 25 minutes", cycle));
        
        if cycle % 4 != 0 {
            countdown(5 * 60, format!("Cycle {}/4: Take a 5 minute break", cycle));
        } else {
            countdown(20 * 60, format!("Cycle {}/4: Take a 20 minute break", cycle));
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
    println!("Starting Pomodoro Timer");
    start_pomodoro();
    println!("Pomodoro Timer Completed");
}
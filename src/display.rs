use std::io::Write;

pub struct Display {}

impl Display {
    pub fn new() -> Self {
        Self {}
    }

    fn display_message(&self, message: &str) {
        println!("{}", message);
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[H");
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
    }

    pub fn display_timer(&self, message: &str, seconds: u64) {
        let formatted_time = format!("{:02}:{:02}", seconds / 60, seconds % 60);

        self.clear_screen();
        self.display_message(message);
        self.display_message(&format!("Time remaining: {}", formatted_time));
        self.flush();
    }
}
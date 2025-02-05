use super::display::Display;

pub struct Timer {
    display: Display,
}

impl Timer {
    pub fn new(display: Display) -> Self {
        Self { display }
    }

    pub fn start(&self, message: &str, minute: u64) {
        for remaining in (1..=minute*60).rev() {
            self.display.display_timer(message, remaining);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
use super::display::Display;
use super::notify::Notify;

pub struct Timer {
    display: Display,
    notify: Notify,
}

impl Timer {
    pub fn new(display: Display, notify: Notify) -> Self {
        Self { display, notify }
    }

    pub fn start(&self, message: &str, minute: u64) {
        for remaining in (1..=minute*60).rev() {
            self.notify.notify(message);
            self.display.display_timer(message, remaining);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
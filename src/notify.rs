use notify_rust::Notification;

pub struct Notify {}

impl Notify {
    pub fn new() -> Self {
        Self {}
    }

    pub fn notify(&self, message: &str) {
        let _ = Notification::new()
            .summary("Pomodoro")
            .body(message)
            .show();
    }
}
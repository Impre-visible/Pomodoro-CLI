use notify_rust::Notification;

pub struct Notify {}

impl Notify {
    pub fn notify(message: &str) {
        let _ = Notification::new()
            .summary("Pomodoro")
            .body(message)
            .show();
    }
}
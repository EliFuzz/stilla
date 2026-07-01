use notify_rust::Notification;

pub fn notify(title: &str, body: &str) {
    Notification::new().summary(title).body(body).show().ok();
}

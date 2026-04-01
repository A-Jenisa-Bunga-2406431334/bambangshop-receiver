use lazy_static::lazy_static;
use rocket::tokio::sync::RwLock;
use crate::model::notification::Notification;

lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn add(notification: Notification) -> Notification {
        let mut data = rocket::tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(NOTIFICATIONS.write());
        data.push(notification.clone());
        return notification;
    }

    pub fn list_all_as_string() -> Vec<String> {
        let data = rocket::tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(NOTIFICATIONS.read());
        return data.iter().map(|f: &Notification| format!("{}", f.clone())).collect();
    }
}
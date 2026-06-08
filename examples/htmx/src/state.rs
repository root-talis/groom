use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub motd: Arc<Mutex<MessageOfTheDay>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            motd: Arc::new(Mutex::new(MessageOfTheDay {
                message: String::from("Welcome to your message of the day."),
                shown: 0,
            })),
        }
    }
}

#[derive(Debug)]
pub struct MessageOfTheDay {
    pub message: String,
    pub shown: u32,
}

use std::sync::{Arc, Mutex};

pub struct AppState {
    pub running: Arc<Mutex<bool>>,
    pub closed: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            running: Arc::new(Mutex::new(false)),
            closed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn stop(&self) {
        let mut is_running = self.closed.lock().unwrap();
        *is_running = true;
    }
}

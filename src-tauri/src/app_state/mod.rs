use std::sync::{Arc, Mutex};

pub struct PatternPart {
    pub x: i32,
    pub y: i32,
    pub delay: u64,
}

pub struct AppState {
    pub closed: Arc<Mutex<bool>>,
    pub running: Arc<Mutex<bool>>,
    pub pattern: Arc<Mutex<Vec<PatternPart>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            closed: Arc::new(Mutex::new(false)),
            running: Arc::new(Mutex::new(false)),
            pattern: Arc::new(Mutex::new(vec![
                PatternPart {
                    x: 10,
                    y: 0,
                    delay: 3000,
                },
                PatternPart {
                    x: 0,
                    y: 10,
                    delay: 3000,
                },
                PatternPart {
                    x: -10,
                    y: 0,
                    delay: 3000,
                },
                PatternPart {
                    x: 0,
                    y: -10,
                    delay: 3000,
                },
            ])),
        }
    }

    pub fn stop(&self) {
        let mut closed = self.closed.lock().unwrap();
        *closed = true;
    }
}

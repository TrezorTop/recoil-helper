use crate::json;
use crate::json::Config;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub closed: Arc<Mutex<bool>>,
    pub running: Arc<Mutex<bool>>,
    pub pattern: Arc<Mutex<Vec<PatternPart>>>,
    pub pattern_name: String,
    pub config: Config,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PatternPart {
    pub x: i32,
    pub y: i32,
    pub delay: u64,
}

impl AppState {
    pub fn new() -> Self {
        let config = json::read_config("config.json").unwrap();

        AppState {
            closed: Arc::new(Mutex::new(false)),
            running: Arc::new(Mutex::new(false)),
            pattern: Arc::new(Mutex::new(vec![])),
            pattern_name: String::from(""),
            config,
        }
    }

    pub fn set_pattern(&mut self, name: String) {
        let mut pattern = self.pattern.lock().unwrap();
        
        pattern.clone_from(self.config.patterns.get(&name).unwrap());
        
        self.pattern_name = name;
    }

    pub fn stop(&self) {
        let mut closed = self.closed.lock().unwrap();
        *closed = true;
    }
}

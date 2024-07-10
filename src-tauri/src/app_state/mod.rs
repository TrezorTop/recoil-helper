use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::json;
use crate::json::Config;

pub struct AppState {
    pub closed: Arc<Mutex<bool>>,
    pub pattern: Arc<Mutex<Vec<PatternPart>>>,
    pub pattern_name: String,
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternPart {
    pub x: i32,
    pub y: i32,
    pub delay: u64,
}

impl AppState {
    pub fn new() -> Self {
        let config = json::read_config().unwrap();

        AppState {
            closed: Arc::new(Mutex::new(false)),
            pattern: Arc::new(Mutex::new(vec![])),
            pattern_name: String::from(""),
            config,
        }
    }

    pub fn reload_config(&mut self) {
        let config = json::read_config().unwrap();

        self.config = config;

        self.set_pattern(self.pattern_name.clone());
    }

    pub fn set_pattern(&mut self, name: String) {
        let mut pattern = self.pattern.lock().unwrap();

        match self.config.patterns.get(&name) {
            None => {
                pattern.clear();
                self.pattern_name.clear();
            }
            Some(p) => {
                *pattern = p.to_owned();
                self.pattern_name = name;
            }
        }
    }

    pub fn stop(&self) {
        let mut closed = self.closed.lock().unwrap();
        *closed = true;
    }
}

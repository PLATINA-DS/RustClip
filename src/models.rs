use serde::{Deserialize, Serialize};
use std::fs;

const IMG_DIR: &str = "/tmp/rustclip_data/images";
const HISTORY_FILE: &str = "/tmp/rustclip_data/history.json";
const MAX_HISTORY_SIZE: usize = 50;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ClipType {
    Text(String),
    Link(String),
    File(String),
    Image(String),
}

#[derive(Serialize, Deserialize, Default)]
pub struct ClipHistory {
    pub items: Vec<ClipType>,
}

impl ClipHistory {
    pub fn load() -> Self {
        let _ = fs::create_dir_all(IMG_DIR);
        if let Ok(data) = fs::read_to_string(HISTORY_FILE) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(HISTORY_FILE, data);
        }
    }

    pub fn add(&mut self, item: ClipType) {
        self.items.retain(|x| x != &item);
        self.items.insert(0, item);
        if self.items.len() > MAX_HISTORY_SIZE {
            self.items.truncate(MAX_HISTORY_SIZE);
        }
        self.save();
    }
}

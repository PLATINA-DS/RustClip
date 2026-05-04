use crate::models::{ClipHistory, ClipType};
use crate::utils::save_image_to_disk;
use arboard::Clipboard;
use std::thread;
use std::time::Duration;

pub fn run_daemon() {
    println!("RustClip daemon started (Text + Files + Images)");
    let mut clipboard = Clipboard::new().unwrap();
    let mut last_text = String::new();
    let mut last_img_hash = 0usize;

    loop {
        if let Ok(text) = clipboard.get_text() {
            let text = text.trim().to_string();
            if !text.is_empty() && text != last_text {
                let mut history = ClipHistory::load();
                if text.starts_with("http") {
                    history.add(ClipType::Link(text.clone()));
                } else if text.starts_with("file://") {
                    history.add(ClipType::File(text.clone()));
                } else {
                    history.add(ClipType::Text(text.clone()));
                }
                last_text = text;
            }
        }

        if let Ok(img) = clipboard.get_image() {
            let current_hash = img.bytes.len() + img.width * img.height;
            if current_hash != last_img_hash {
                if let Some(path) = save_image_to_disk(&img) {
                    let mut history = ClipHistory::load();
                    history.add(ClipType::Image(path));
                    last_img_hash = current_hash;
                }
            }
        }

        thread::sleep(Duration::from_millis(800));
    }
}

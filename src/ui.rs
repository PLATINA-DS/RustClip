use crate::models::{ClipHistory, ClipType};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_show() {
    let history = ClipHistory::load();
    if history.items.is_empty() { return; }

    let rofi_input = history.items.iter().map(|item| {
        match item {
            ClipType::Text(t) => {
                format!("[TEXT] {}\0icon\x1fedit-paste", t.replace('\n', " "))
            },
            ClipType::Link(l) => {
                format!("[LINK] {}\0icon\x1fbrowser", l)
            },
            ClipType::File(f) => {
                format!("[FILE] {}\0icon\x1fsystem-file-manager", f.replace("file://", ""))
            },
            ClipType::Image(p) => {
                let name = std::path::Path::new(p).file_name().unwrap().to_str().unwrap();
                format!("[IMAGE] {}\0icon\x1f{}", name, p)
            }
        }
    }).collect::<Vec<_>>().join("\n");

    let mut child = Command::new("rofi")
        .args([
            "-dmenu",
            "-i",
            "-p", "Clipboard:",
            "-l", "12",
            "-show-icons"
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("Rofi failed");

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(rofi_input.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if let Some(found) = history.items.iter().find(|item| {
            match item {
                ClipType::Text(t) => format!("[TEXT] {}", t.replace('\n', " ")) == selected,
                ClipType::Link(l) => format!("[LINK] {}", l) == selected,
                ClipType::File(f) => format!("[FILE] {}", f.replace("file://", "")) == selected,
                ClipType::Image(p) => format!("[IMAGE] {}", std::path::Path::new(p).file_name().unwrap().to_str().unwrap()) == selected,
            }
        }) {
            let mut cb = arboard::Clipboard::new().unwrap();
            match found {
                ClipType::Text(t) | ClipType::Link(t) | ClipType::File(t) => cb.set_text(t.clone()).unwrap(),
                ClipType::Image(p) => {
                    if let Ok(img_data) = image::open(p) {
                        let rgba = img_data.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        cb.set_image(arboard::ImageData {
                            width: w as usize,
                            height: h as usize,
                            bytes: std::borrow::Cow::Owned(rgba.into_raw()),
                        }).unwrap();
                    }
                }
            }
        }
    }
}

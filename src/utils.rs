use arboard::ImageData;
use image;
use std::time::{SystemTime, UNIX_EPOCH};

const IMG_DIR: &str = "/tmp/rustclip_data/images";

pub fn save_image_to_disk(img: &ImageData) -> Option<String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let file_name = format!("{}/img_{}.png", IMG_DIR, timestamp);

    image::save_buffer(
        &file_name,
        &img.bytes,
        img.width as u32,
        img.height as u32,
        image::ColorType::Rgba8,
    ).ok()?;

    Some(file_name)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VideoMetadata {
    pub path: String,
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub video_codec: String,
    pub format: String,
    pub size_bytes: u64,
    pub duration_secs: f64,
}

#[derive(Serialize)]
pub struct Video {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_secs: f64,
    pub cover_path: Option<String>,
}

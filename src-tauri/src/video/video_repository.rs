use crate::video::model::VideoMetadata;

use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn save_video(
    pool: State<'_, SqlitePool>,
    video: VideoMetadata,
    name: String,
    type_: String,
    year: i32,
) -> Result<String, String> {
    sqlx::query(
    "INSERT INTO Videos (filename, path, width, height, fps, video_codec, format, size_bytes, duration_secs, date_added, type, year)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&name)
    .bind(&video.path)
    .bind(video.width)
    .bind(video.height)
    .bind(video.fps)
    .bind(&video.video_codec)
    .bind(&video.format)
    .bind(video.size_bytes as i64)
    .bind(video.duration_secs)
    .bind(chrono::Utc::now().timestamp())
    .bind(&type_)
    .bind(year)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok("Video guardado".to_string())
}

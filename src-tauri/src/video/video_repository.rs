use crate::video::model::VideoMetadata;

use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn save_video(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    video: VideoMetadata,
    name: String,
    type_: String,
    year: i32,
    cover_path: Option<String>,
) -> Result<String, String> {
    let stored_cover_path = match cover_path
        .as_deref()
        .filter(|path| !path.trim().is_empty())
    {
        Some(path) => Some(store_cover_image(&app, path).await?),
        None => None,
    };

    sqlx::query(
    "INSERT INTO Videos (filename, path, width, height, fps, video_codec, format, size_bytes, duration_secs, date_added, type, year, cover_path)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
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
    .bind(stored_cover_path.as_deref())
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok("Video guardado".to_string())
}

async fn store_cover_image(app: &AppHandle, original_path: &str) -> Result<String, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let covers_dir = app_data_dir.join("covers");
    let timestamp = chrono::Utc::now().timestamp_millis();
    let extension = std::path::Path::new(original_path)
        .extension()
        .and_then(|ext| ext.to_str());
    let file_name = match extension {
        Some(ext) => format!("cover_{timestamp}.{ext}"),
        None => format!("cover_{timestamp}"),
    };
    let destination = covers_dir.join(file_name);

    let source = original_path.to_string();
    let destination_clone = destination.clone();
    let covers_dir_clone = covers_dir.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        std::fs::create_dir_all(&covers_dir_clone).map_err(|e| e.to_string())?;
        std::fs::copy(&source, &destination_clone).map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(destination.to_string_lossy().to_string())
}

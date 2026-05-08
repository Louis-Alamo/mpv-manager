use sqlx::{Row, SqlitePool};
use tauri::Manager;

mod video;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let db_path = app.path().app_data_dir().unwrap().join("videos.db");
            std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
            let db_url = format!("sqlite:{}?mode=rwc", db_path.to_str().unwrap());

            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePool::connect(&db_url).await.unwrap();

                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS Videos (
                        id INTEGER PRIMARY KEY,
                        filename TEXT NOT NULL,
                        path TEXT NOT NULL,
                        width INTEGER,
                        height INTEGER,
                        fps REAL,
                        video_codec TEXT,
                        format TEXT,
                        size_bytes INTEGER NOT NULL,
                        duration_secs REAL NOT NULL,
                        date_added INTEGER,
                        type TEXT,
                        year INTEGER,
                        cover_path TEXT
                    )",
                )
                .execute(&pool)
                .await
                .expect("Failed to create Videos table");

                let columns = sqlx::query("PRAGMA table_info(Videos)")
                    .fetch_all(&pool)
                    .await
                    .expect("Failed to read Videos schema");
                let column_names: Vec<String> = columns
                    .iter()
                    .map(|row| row.get::<String, _>("name"))
                    .collect();

                let mut migrations: Vec<&str> = Vec::new();
                if !column_names.iter().any(|name| name == "type") {
                    migrations.push("ALTER TABLE Videos ADD COLUMN type TEXT");
                }
                if !column_names.iter().any(|name| name == "year") {
                    migrations.push("ALTER TABLE Videos ADD COLUMN year INTEGER");
                }
                if !column_names.iter().any(|name| name == "cover_path") {
                    migrations.push("ALTER TABLE Videos ADD COLUMN cover_path TEXT");
                }

                for statement in migrations {
                    sqlx::query(statement)
                        .execute(&pool)
                        .await
                        .expect("Failed to migrate Videos table");
                }

                pool
            });

            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            video::video_service::get_video_metadata,
            video::video_repository::save_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

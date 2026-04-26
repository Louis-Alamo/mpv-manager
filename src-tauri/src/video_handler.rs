#[tauri::command]
pub fn get_video_metadata() -> String {
    "Hola desde rust".to_string()
}

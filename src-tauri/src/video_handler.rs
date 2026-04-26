use serde::Serialize;

#[derive(Serialize)]
pub struct VideoMetadata {
    path: String,
    filename: String,
    width: u32,
    height: u32,
    fps: f64,
    video_codec: String,
    format: String,
    size_bytes: u64,
    duration_secs: f64,
}

#[tauri::command]
pub fn get_video_metadata(path: String) -> Result<VideoMetadata, String> {
    let info = ffprobe::ffprobe(&path).unwrap();

    let stream = info
        .streams
        .iter()
        .find(|s| s.codec_type == Some("video".to_string()))
        .unwrap();

    let filename = std::path::Path::new(&path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let width = stream.width.unwrap() as u32;
    let height = stream.height.unwrap() as u32;

    let fps = stream.r_frame_rate.split('/').collect::<Vec<&str>>();
    let fps_value = fps[0].parse::<f64>().unwrap() / fps[1].parse::<f64>().unwrap();

    let video_codec = stream.codec_name.clone().unwrap();

    let format = info.format.format_name.clone();
    let size_bytes = info.format.size.parse::<u64>().unwrap();
    let duration_secs = info
        .format
        .duration
        .clone()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    Ok(VideoMetadata {
        path,
        filename,
        width,
        height,
        fps: fps_value,
        video_codec,
        format,
        size_bytes,
        duration_secs,
    })
}

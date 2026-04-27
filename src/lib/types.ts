export interface VideoMetadata {
  path: string;
  filename: string;
  width: number;
  height: number;
  fps: number;
  video_codec: string;
  format: string;
  size_bytes: number;
  duration_secs: number;
}

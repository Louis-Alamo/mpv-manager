CREATE TABLE Videos (
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
    date_added INTEGER
)

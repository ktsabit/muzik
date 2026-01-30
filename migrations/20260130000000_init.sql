CREATE TABLE IF NOT EXISTS roots (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    path TEXT NOT NULL UNIQUE, 
    last_scanned_at DATETIME DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album TEXT NOT NULL,
    root_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    FOREIGN KEY (root_id) REFERENCES roots(id) ON DELETE CASCADE
);

-- EAV table
CREATE TABLE IF NOT EXISTS metadata (
    track_id TEXT NOT NULL,
    meta_key TEXT NOT NULL,
    meta_value TEXT NOT NULL,
    FOREIGN KEY (track_id) REFERENCES tracks(id) ON DELETE CASCADE,
    UNIQUE (track_id, meta_key)
);


CREATE INDEX IF NOT EXISTS idx_tracks_title ON tracks(title);
CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);
CREATE INDEX IF NOT EXISTS idx_tracks_album ON tracks(album);
CREATE INDEX IF NOT EXISTS idx_tracks_root ON tracks(root_id);
CREATE INDEX IF NOT EXISTS idx_metadata_key ON metadata(meta_key);





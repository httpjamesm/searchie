-- Add up migration script here
CREATE TABLE datapoint_chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    datapoint_id INTEGER NOT NULL,
    data BLOB NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
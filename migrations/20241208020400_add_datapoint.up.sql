-- Add up migration script here
CREATE TABLE datapoints (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dataset_id TEXT NOT NULL REFERENCES datasets(id),
    data_type TEXT NOT NULL,
    data BLOB NOT NULL,
    hash TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add up migration script here
CREATE TABLE datapoint_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    datapoint_id INTEGER NOT NULL REFERENCES datapoints(id),
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

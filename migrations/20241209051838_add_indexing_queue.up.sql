-- Add up migration script here
CREATE TABLE indexing_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    datapoint_id INTEGER NOT NULL REFERENCES datapoints(id),
    status TEXT NOT NULL DEFAULT 'pending',
    error TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME
); 
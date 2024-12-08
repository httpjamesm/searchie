-- Add up migration script here
ALTER TABLE datapoints ADD COLUMN indexed_at DATETIME;

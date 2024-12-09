-- Up migration
ALTER TABLE datapoints ADD COLUMN hash TEXT;
-- Update existing rows with unique values
-- UPDATE datapoints SET hash = generate_hash();
CREATE UNIQUE INDEX idx_datapoints_hash ON datapoints(hash);

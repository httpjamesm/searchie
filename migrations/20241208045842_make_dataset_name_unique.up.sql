-- Add up migration script here
ALTER TABLE datasets ADD CONSTRAINT dataset_name_key UNIQUE (name);
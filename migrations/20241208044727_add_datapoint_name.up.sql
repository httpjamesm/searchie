-- Add up migration script here
alter table datapoints add column name text not null;
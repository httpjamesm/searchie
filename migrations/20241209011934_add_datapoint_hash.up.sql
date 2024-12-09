-- Add up migration script here
alter table datapoints add column hash text not null;
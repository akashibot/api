-- Add migration script here
alter table tags
    add column id uuid primary key default gen_random_uuid ();
-- Add migration script here
alter table guilds
    alter column created_at type timestamp;

alter table users
    alter column created_at type timestamp;

alter table tags
    alter column created_at type timestamp;
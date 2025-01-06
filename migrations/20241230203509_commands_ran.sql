-- Add migration script here
alter table guilds
    add column commands_ran bigint default 0;
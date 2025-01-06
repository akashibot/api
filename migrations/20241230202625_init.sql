-- thanks database.build

create table guilds (
                        id text primary key,
                        name text not null,
                        owner_id text not null,
                        created_at timestamptz default now()
);

create table users (
                       id text primary key,
                       username text not null,
                       discriminator text not null,
                       joined_at timestamptz default now()
);

create table tags (
                      id bigint primary key generated always as identity,
                      guild_id text references guilds (id),
                      name text not null,
                      content text not null,
                      created_by text references users (id),
                      created_at timestamptz default now()
);

alter table guilds
    drop owner_id;

alter table users
    drop discriminator;

alter table users
    drop username;

alter table users
    rename column joined_at to created_at;
-- Add migration script here
-- Create the guilds table
CREATE TABLE guilds (
    id TEXT PRIMARY KEY,
    plan TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
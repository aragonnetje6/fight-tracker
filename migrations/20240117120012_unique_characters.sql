-- Add migration script here
SET CONSTRAINTS ALL IMMEDIATE;
ALTER TABLE character ADD CONSTRAINT game_character_unique UNIQUE (name, game_id);

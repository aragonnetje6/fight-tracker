-- Add migration script here
CREATE TABLE game (
    id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE character (
    id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text NOT NULL,
    game_id integer REFERENCES game (id) NOT NULL 
);

CREATE TYPE matchResult AS ENUM ('win', 'loss', 'draw');

CREATE TABLE match (
    id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    result matchResult NOT NULL
)

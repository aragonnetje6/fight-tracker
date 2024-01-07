-- Add migration script here
ALTER TABLE match ADD COLUMN timestamp timestamptz;
ALTER TABLE match ADD COLUMN character_id integer;
ALTER TABLE match ALTER COLUMN character_id SET NOT NULL;
ALTER TABLE match ADD FOREIGN KEY (character_id) REFERENCES character (id);

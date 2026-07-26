ALTER TABLE fight_instances
ADD COLUMN multiplication INTEGER NOT NULL DEFAULT 1;

ALTER TABLE fight_instances
ADD COLUMN entry_cost TEXT NOT NULL DEFAULT '';

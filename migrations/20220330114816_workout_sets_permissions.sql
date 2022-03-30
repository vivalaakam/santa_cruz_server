ALTER TABLE workout_sets
    ADD COLUMN permissions JSONB NOT NULL DEFAULT '{}';

ALTER TABLE workout_repeats
    ADD COLUMN permissions JSONB NOT NULL DEFAULT '{}';

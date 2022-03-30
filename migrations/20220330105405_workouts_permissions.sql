ALTER TABLE workouts
    ADD COLUMN permissions JSONB NOT NULL DEFAULT '{}';


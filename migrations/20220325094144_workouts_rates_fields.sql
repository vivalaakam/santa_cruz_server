-- Add migration script here
ALTER TABLE workouts
    ADD comment TEXT DEFAULT '';

ALTER TABLE workouts
    ADD rate INT DEFAULT 0;

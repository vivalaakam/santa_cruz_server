-- Add migration script here
ALTER TABLE workouts
    ALTER COLUMN status TYPE VARCHAR;

UPDATE workouts SET status = 'unknown' WHERE status = '0';
UPDATE workouts SET status = 'inProgress' WHERE status = '1';
UPDATE workouts SET status = 'finished' WHERE status = '2';

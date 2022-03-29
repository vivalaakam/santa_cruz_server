CREATE TABLE IF NOT EXISTS workout_repeats
(
    id             SERIAL PRIMARY KEY,
    created_at     timestamptz NOT NULL DEFAULT NOW()::timestamp,
    updated_at     timestamptz NOT NULL DEFAULT NOW()::timestamp,
    workout_set_id INT         NOT NULL,
    exercise_id    INT         NOT NULL,
    repeats        INT         NOT NULL DEFAULT 0,
    weight         DOUBLE PRECISION     DEFAULT 0,
    time           DOUBLE PRECISION     DEFAULT 0
);

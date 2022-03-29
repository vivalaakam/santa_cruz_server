CREATE TABLE IF NOT EXISTS workout_sets
(
    id         SERIAL PRIMARY KEY,
    workout_id INTEGER     NOT NULL,
    position   INTEGER              DEFAULT 0,
    type       JSONB       NOT NULL DEFAULT '{}'::JSONB,
    comment    TEXT                 DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT NOW():: timestamp,
    updated_at timestamptz NOT NULL DEFAULT NOW():: timestamp
);

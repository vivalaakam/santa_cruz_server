CREATE TABLE IF NOT EXISTS workouts
(
    id         SERIAL PRIMARY KEY,
    status     INTEGER     NOT NULL,
    day        timestamptz NOT NULL DEFAULT NOW()::timestamp,
    created_at timestamptz NOT NULL DEFAULT NOW()::timestamp,
    updated_at timestamptz NOT NULL DEFAULT NOW()::timestamp
);

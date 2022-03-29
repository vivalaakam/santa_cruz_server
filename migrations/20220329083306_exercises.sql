CREATE TABLE IF NOT EXISTS exercises
(
    id          SERIAL PRIMARY KEY,
    created_at  timestamptz NOT NULL DEFAULT NOW():: timestamp,
    updated_at  timestamptz NOT NULL DEFAULT NOW():: timestamp,
    name        TEXT        NOT NULL,
    description TEXT        NOT NULL DEFAULT ''
);

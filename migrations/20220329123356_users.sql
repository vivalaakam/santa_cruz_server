CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT NOW()::timestamp,
    updated_at timestamptz NOT NULL DEFAULT NOW()::timestamp,
    email      TEXT        NOT NULL UNIQUE,
    password   TEXT        NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions
(
    id          SERIAL PRIMARY KEY,
    created_at  timestamptz NOT NULL DEFAULT NOW()::timestamp,
    updated_at  timestamptz NOT NULL DEFAULT NOW()::timestamp,
    user_id     INT         NOT NULL,
    token       TEXT        NOT NULL UNIQUE,
    device_name TEXT        NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id              VARCHAR(36)     PRIMARY KEY NOT NULL,
    name            TEXT            UNIQUE NOT NULL,
    password        TEXT            NOT NULL
)
CREATE TABLE IF NOT EXISTS users (
    id              TEXT    PRIMARY KEY NOT NULL,
    name            TEXT    UNIQUE NOT NULL,
    password        TEXT    NOT NULL
)
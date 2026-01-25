CREATE TABLE IF NOT EXISTS users (
    id                  VARCHAR(36)     UNIQUE NOT NULL,
    name                VARCHAR(255)    UNIQUE NOT NULL,
    password            VARCHAR(255)    NOT NULL
)
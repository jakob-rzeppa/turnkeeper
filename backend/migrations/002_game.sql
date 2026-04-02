CREATE TABLE IF NOT EXISTS games (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    name                    TEXT                UNIQUE NOT NULL,
    gm_user_id              VARCHAR(36)         NOT NULL
)
CREATE TABLE IF NOT EXISTS games (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    name                    TEXT                UNIQUE NOT NULL,
    round_number            INTEGER             NOT NULL,
    current_player_index    INTEGER             NOT NULL
)
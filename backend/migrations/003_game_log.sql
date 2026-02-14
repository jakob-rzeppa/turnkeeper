CREATE TABLE IF NOT EXISTS games_log (
    game_id             VARCHAR(36)         NOT NULL,
    timestamp           DATETIME            NOT NULL DEFAULT NOW,

    event                TEXT                NOT NULL
)
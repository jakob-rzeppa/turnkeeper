CREATE TABLE IF NOT EXISTS game_instances (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    
    serialized              TEXT                NOT NULL
);
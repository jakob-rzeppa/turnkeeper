CREATE TABLE IF NOT EXISTS players (
    id          VARCHAR(36)    PRIMARY KEY NOT NULL,
    game_id     VARCHAR(36)    NOT NULL,
    user_id     VARCHAR(36)    NOT NULL,
    position    INTEGER        NOT NULL,

    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT,

    UNIQUE (game_id, position),
    UNIQUE (game_id, user_id)
)
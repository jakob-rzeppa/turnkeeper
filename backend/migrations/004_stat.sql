CREATE TABLE IF NOT EXISTS stats (
    id              VARCHAR(36)     PRIMARY KEY NOT NULL,
    player_id       VARCHAR(36)     NOT NULL,
    key             TEXT            NOT NULL,
    kind            VARCHAR(255)    NOT NULL,
    number_value    INTEGER,
    string_value    TEXT,
    boolean_value   BOOLEAN,

    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,

    UNIQUE (kind, player_id)
)
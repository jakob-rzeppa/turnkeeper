CREATE TABLE IF NOT EXISTS game_instances (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    name                    VARCHAR(255)        NOT NULL,

    current_player_index    INT                 NOT NULL,
    round                   INT                 NOT NULL,

    gm_user_id              VARCHAR(36)         NOT NULL,

    source_game             VARCHAR(36)         NOT NULL,

    created_at              DATETIME            NOT NULL,
    last_played_at          DATETIME            NOT NULL,

    FOREIGN KEY (gm_user_id) REFERENCES users(id) ON DELETE RESTRICT
    FOREIGN KEY (source_game) REFERENCES games(id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS game_stats (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,
    name                    VARCHAR(255)        NOT NULL,

    int_value               INT                 ,
    float_value             FLOAT               ,
    string_value            TEXT                ,
    bool_value              BOOLEAN             ,

    default_int_value       INT                 ,
    default_float_value     FLOAT               ,
    default_string_value    TEXT                ,
    default_bool_value      BOOLEAN             ,

    visibility              VARCHAR(20)         NOT NULL,

    pos                     VARCHAR(255)        NOT NULL,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS player_stats (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,
    name                    VARCHAR(255)        NOT NULL,

    default_int_value       INT                 ,
    default_float_value     FLOAT               ,
    default_string_value    TEXT                ,
    default_bool_value      BOOLEAN             ,

    visibility              VARCHAR(20)         NOT NULL,

    pos                     VARCHAR(255)        NOT NULL,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS player_stat_values (
    player_stat_id          VARCHAR(36)         NOT NULL,
    player_id               VARCHAR(36)         NOT NULL,

    int_value               INT                 ,
    float_value             FLOAT               ,
    string_value            TEXT                ,
    bool_value              BOOLEAN             ,

    PRIMARY KEY (player_stat_id, player_id),
    FOREIGN KEY (player_stat_id) REFERENCES player_stats(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS actions (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,
    name                    VARCHAR(255)        NOT NULL,

    source_code             TEXT                NOT NULL,
    pos                     VARCHAR(255)        NOT NULL,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pages (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,
    name                    VARCHAR(255)        NOT NULL,

    source_code             TEXT                NOT NULL,
    pos                     VARCHAR(255)        NOT NULL,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS players (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,
    user_id                 VARCHAR(36)         ,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS logs (
    id                      VARCHAR(36)          PRIMARY KEY NOT NULL,
    game_instance_id        VARCHAR(36)         NOT NULL,

    FOREIGN KEY (game_instance_id) REFERENCES game_instances(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS log_entries (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    log_id                  VARCHAR(36)         NOT NULL,
    entry                   TEXT                NOT NULL,
    timestamp               DATETIME            NOT NULL,

    FOREIGN KEY (log_id) REFERENCES logs(id) ON DELETE CASCADE
);
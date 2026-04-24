CREATE TABLE IF NOT EXISTS games (
    id                      VARCHAR(36)         PRIMARY KEY NOT NULL,
    name                    VARCHAR(255)        NOT NULL,
    description             TEXT                NOT NULL,

    source_code             TEXT                NOT NULL,
    
    created_at              DATETIME            NOT NULL,
    updated_at              DATETIME            NOT NULL
)
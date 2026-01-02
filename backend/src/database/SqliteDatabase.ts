import Database from 'better-sqlite3';

import config from '../config/config.js';

export class SqliteDatabase extends Database {
    private static instance: null | SqliteDatabase = null;

    private constructor() {
        if (!config.dbPath) {
            throw new Error('Database path is not defined in the configuration.');
        }

        super(config.dbPath);

        this.initializeTables();
    }
    public static getInstance(): SqliteDatabase {
        SqliteDatabase.instance ??= new SqliteDatabase();

        return SqliteDatabase.instance;
    }

    public dropTables() {
        // First drop tables with foreign keys
        this.exec('DROP TABLE IF EXISTS player_stats');
        this.exec('DROP TABLE IF EXISTS player_tradables');
        this.exec('DROP TABLE IF EXISTS messages');
        this.exec('DROP TABLE IF EXISTS player_order');

        // Then drop the other tables
        this.exec('DROP TABLE IF EXISTS tradables');
        this.exec('DROP TABLE IF EXISTS players');
        this.exec('DROP TABLE IF EXISTS game_state');
    }

    public initializeTables() {
        this.exec(
            `CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY, 
                name TEXT UNIQUE NOT NULL, 
                secret TEXT NOT NULL, 
                notes TEXT NOT NULL DEFAULT "", 
                hidden_notes TEXT NOT NULL DEFAULT ""
            )`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS player_stats (
                id INTEGER PRIMARY KEY, 
                player_id INT NOT NULL, 
                name TEXT NOT NULL, 
                type TEXT NOT NULL CHECK(type IN ('string', 'number', 'boolean')), 
                value TEXT NOT NULL DEFAULT "", 
                
                FOREIGN KEY (player_id) REFERENCES players (id)
            )`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS tradables (
                id INTEGER PRIMARY KEY, 
                -- Not allowing empty or whitespace-only names
                name TEXT UNIQUE NOT NULL CHECK(length(trim(name)) > 0),
                initial_quantity INT NOT NULL DEFAULT 0
            )`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS player_tradables (
                id INTEGER PRIMARY KEY, 
                player_id INT NOT NULL, 
                tradable_id INT NOT NULL, 
                quantity INT NOT NULL DEFAULT 0,
                 
                FOREIGN KEY (player_id) REFERENCES players (id),
                FOREIGN KEY (tradable_id) REFERENCES tradables (id),
                UNIQUE(player_id, tradable_id)
            )`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY, 
                player_id INT NOT NULL, 
                send_by TEXT NOT NULL CHECK(send_by IN ('player', 'gm')), 
                content TEXT NOT NULL CHECK(length(trim(content)) > 0), 
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP, 

                FOREIGN KEY (player_id) REFERENCES players (id)
            )`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS game_state (
                id INTEGER PRIMARY KEY, 
                round_number INT NOT NULL DEFAULT 0, 
                current_player_index INT NOT NULL DEFAULT 0,
                notes TEXT NOT NULL DEFAULT "", 
                hidden_notes TEXT NOT NULL DEFAULT ""
            )`,
        );

        this.exec(`
            CREATE TABLE IF NOT EXISTS player_order (
                id INTEGER PRIMARY KEY,
                game_state_id INT NOT NULL,
                player_id INT NOT NULL,
                position INT NOT NULL,
                
                FOREIGN KEY (game_state_id) REFERENCES game_state (id),
                FOREIGN KEY (player_id) REFERENCES players (id),
                UNIQUE(position, game_state_id),
                UNIQUE(player_id, game_state_id)
            )
        `);
    }
}

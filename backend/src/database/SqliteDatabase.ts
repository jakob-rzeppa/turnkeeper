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
        this.exec('DROP TABLE IF EXISTS player_stats');
        this.exec('DROP TABLE IF EXISTS players');
        this.exec('DROP TABLE IF EXISTS game_state');
    }

    public initializeTables() {
        this.exec(
            'CREATE TABLE IF NOT EXISTS players (id INTEGER PRIMARY KEY, name TEXT UNIQUE NOT NULL, secret TEXT NOT NULL, notes TEXT NOT NULL DEFAULT "", hidden_notes TEXT NOT NULL DEFAULT "")',
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS player_stats (id INTEGER PRIMARY KEY, player_id INT NOT NULL, name TEXT NOT NULL, type TEXT NOT NULL CHECK(type IN ('string', 'number', 'boolean')), value TEXT NOT NULL DEFAULT "", FOREIGN KEY (player_id) REFERENCES players (id))`,
        );

        this.exec(
            `CREATE TABLE IF NOT EXISTS game_state (id INTEGER PRIMARY KEY, round_number INT NOT NULL, current_player_index INT NOT NULL, player_order TEXT NOT NULL, notes TEXT NOT NULL DEFAULT "", hidden_notes TEXT NOT NULL DEFAULT "")`,
        );
    }
}

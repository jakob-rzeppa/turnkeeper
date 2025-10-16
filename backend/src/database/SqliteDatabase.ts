import Database from "better-sqlite3";
import config from "../config/config";

export class SqliteDatabase extends Database {
    private static instance: SqliteDatabase;

    public static getInstance(): SqliteDatabase {
        if (!SqliteDatabase.instance) {
            SqliteDatabase.instance = new SqliteDatabase();
        }
        return SqliteDatabase.instance;
    }
    private constructor() {
        if (!config.dbPath) {
            throw new Error(
                "Database path is not defined in the configuration."
            );
        }

        super(config.dbPath);

        this.initializeTables();
    }

    public initializeTables() {
        this.exec(
            "CREATE TABLE IF NOT EXISTS stats (id INTEGER PRIMARY KEY, playerId INT NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL, FOREIGN KEY (playerId) REFERENCES players (id))"
        );

        this.exec(
            "CREATE TABLE IF NOT EXISTS players (id INTEGER PRIMARY KEY, name TEXT UNIQUE NOT NULL, secret TEXT NOT NULL)"
        );
    }

    public dropTables() {
        this.exec("DROP TABLE IF EXISTS stats");
        this.exec("DROP TABLE IF EXISTS players");
    }
}

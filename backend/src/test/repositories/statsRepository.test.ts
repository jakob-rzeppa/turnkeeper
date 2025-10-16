import { beforeEach, describe, expect, it, vi } from "vitest";
import { SqliteDatabase } from "../../database/SqliteDatabase";
import { statsRepository } from "../../repositories/statsRepository";
import logger from "../../services/logger";

// Mock the config to use an in-memory database for testing
vi.mock("../../config/config.ts", () => ({
    default: {
        dbPath: ":memory:",
    },
}));

vi.mock("../../services/logger.ts", () => ({
    default: {
        error: vi.fn(),
    },
}));

describe("Stats Repository", () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe("createStatForAllPlayers", () => {
        it("should create a stat for all players", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')"
            );

            statsRepository.createStatForAllPlayers({
                name: "score",
                value: "100",
            });

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(2);
            expect(stats).toContainEqual({
                id: 1,
                player_id: 1,
                name: "score",
                value: "100",
            });
            expect(stats).toContainEqual({
                id: 2,
                player_id: 2,
                name: "score",
                value: "100",
            });
        });
        it("should not create duplicate stats for players", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')"
            );
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'score', '100')"
            );

            statsRepository.createStatForAllPlayers({
                name: "score",
                value: "200",
            });

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                player_id: 1,
                name: "score",
                value: "100",
            });
        });
    });

    describe("createStatForPlayer", () => {
        it("should create a stat for a specific player", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')"
            );

            statsRepository.createStatForPlayer(1, {
                name: "level",
                value: "5",
            });

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                player_id: 1,
                name: "level",
                value: "5",
            });
        });
        it("should not create duplicate stats for the player", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')"
            );
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'level', '5')"
            );

            statsRepository.createStatForPlayer(1, {
                name: "level",
                value: "10",
            });

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                player_id: 1,
                name: "level",
                value: "5",
            });
        });

        it("should log an error if the player does not exist", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')"
            );

            statsRepository.createStatForPlayer(999, {
                name: "level",
                value: "5",
            });

            expect(logger.error).toHaveBeenCalledWith({
                message: "Player with id 999 not found",
            });
        });
    });

    describe("removeStatFromPlayer", () => {
        it("should remove a stat from a specific player", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')"
            );
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, value) VALUES (1, 1, 'level', '5'), (2, 1, 'score', '100')"
            );

            statsRepository.removeStatFromPlayer(1, 1);

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 2,
                player_id: 1,
                name: "score",
                value: "100",
            });
        });
        it("should do nothing if the stat does not exist for the player", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')"
            );
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, value) VALUES (1, 1, 'level', '5')"
            );

            statsRepository.removeStatFromPlayer(1, 999);

            const stats = db.prepare("SELECT * FROM player_stats").all() as {
                id: number;
                player_id: number;
                name: string;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                player_id: 1,
                name: "level",
                value: "5",
            });
        });
    });
});

import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase';
import { statsRepository } from '../../repositories/statsRepository';
import logger from '../../services/logger';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

vi.mock('../../services/logger.ts', () => ({
    default: {
        error: vi.fn(),
    },
}));

describe('Stats Repository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('createStatForAllPlayers', () => {
        it('should create a stat for all players', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            statsRepository.createStatForAllPlayers({
                name: 'score',
                value: 'test',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(2);
            expect(stats).toContainEqual({
                id: 1,
                name: 'score',
                player_id: 1,
                type: 'string',
                value: 'test',
            });
            expect(stats).toContainEqual({
                id: 2,
                name: 'score',
                player_id: 2,
                type: 'string',
                value: 'test',
            });
        });
        it('should not create duplicate stats for players', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', 'test')",
            );

            statsRepository.createStatForAllPlayers({
                name: 'score',
                value: 'test2',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'score',
                player_id: 1,
                type: 'string',
                value: 'test',
            });
        });

        it('should allow creating stats with no value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForAllPlayers({
                name: 'level',
                value: '',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: '',
            });
        });

        it('should be able to create numeric stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForAllPlayers({
                name: 'level',
                value: 5,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should be able to create boolean stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForAllPlayers({
                name: 'isActive',
                value: true,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'isActive',
                player_id: 1,
                type: 'boolean',
                value: 'true',
            });
        });
    });

    describe('createStatForPlayer', () => {
        it('should create a stat for a specific player', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            statsRepository.createStatForPlayer(1, {
                name: 'level',
                value: 'test',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: 'test',
            });
        });
        it('should not create duplicate stats for the player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'level', 'number', '5')",
            );

            statsRepository.createStatForPlayer(1, {
                name: 'level',
                value: 10,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should log an error if the player does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForPlayer(999, {
                name: 'level',
                value: 5,
            });

            expect(logger.error).toHaveBeenCalledWith({
                message: 'Player with id 999 not found',
            });
        });

        it('should allow creating stats with no value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForPlayer(1, {
                name: 'level',
                value: '',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: '',
            });
        });

        it('should be able to create numeric stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForPlayer(1, {
                name: 'level',
                value: 5,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should be able to create boolean stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForPlayer(1, {
                name: 'isActive',
                value: false,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'isActive',
                player_id: 1,
                type: 'boolean',
                value: 'false',
            });
        });
    });

    describe('updateStatForPlayer', () => {
        it('should update a stat for a specific player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                name: 'level',
                value: 10,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '10',
            });
        });

        it('should do nothing if the stat does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(999, 999, {
                name: 'level',
                value: 10,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should do nothing if the stat exists but for a different player', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(2, 1, {
                name: 'level',
                value: 10,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should not allow updating to a duplicate stat name for the same player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5'), (2, 1, 'score', 'number', '100')",
            );

            statsRepository.updateStatForPlayer(2, 1, {
                name: 'level',
                value: 10,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(2);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
            expect(stats).toContainEqual({
                id: 2,
                name: 'score',
                player_id: 1,
                type: 'number',
                value: '100',
            });
        });

        it('should not allow to update stat name to an empty string', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                name: '',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: 'test',
            });
        });

        it('should allow updating the stat to have no value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                name: 'level',
                value: '',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: '',
            });
        });

        it('should do nothing if no fields are provided to update', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(1, 1, {});

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });

        it('should update stats to a number value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                value: 42,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '42',
            });
        });

        it('should update stats to a boolean value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'isActive', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                value: true,
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'isActive',
                player_id: 1,
                type: 'boolean',
                value: 'true',
            });
        });

        it('should update stats to a string value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(1, 1, {
                value: 'expert',
            });

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'string',
                value: 'expert',
            });
        });
    });

    describe('removeStatFromPlayer', () => {
        it('should remove a stat from a specific player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5'), (2, 1, 'score', 'number', '100')",
            );

            statsRepository.removeStatFromPlayer(1, 1);

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 2,
                name: 'score',
                player_id: 1,
                type: 'number',
                value: '100',
            });
        });
        it('should do nothing if the stat does not exist for the player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.removeStatFromPlayer(1, 999);

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(1);
            expect(stats).toContainEqual({
                id: 1,
                name: 'level',
                player_id: 1,
                type: 'number',
                value: '5',
            });
        });
    });

    describe('removeAllStatsFromPlayer', () => {
        it('should remove all stats from a specific player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5'), (2, 1, 'score', 'number', '100')",
            );

            statsRepository.removeAllStatsFromPlayer(1);

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                type: 'boolean' | 'number' | 'string';
                value: string;
            }[];

            expect(stats).toHaveLength(0);
        });

        it('should do nothing if the player has no stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.removeAllStatsFromPlayer(1);

            const stats = db.prepare('SELECT * FROM player_stats').all() as {
                id: number;
                name: string;
                player_id: number;
                value: string;
            }[];

            expect(stats).toHaveLength(0);
        });
    });
});

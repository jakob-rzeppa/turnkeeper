import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase.js';
import { Conflict, NotFound, ValidationError } from '../../repositories/repositoryErrors.js';
import { statsRepository } from '../../repositories/statsRepository.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
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

            statsRepository.createStatForAllPlayers('score', 'test');

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

            statsRepository.createStatForAllPlayers('score', 'test2');

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

            statsRepository.createStatForAllPlayers('level', '');

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

            statsRepository.createStatForAllPlayers('level', 5);

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

            statsRepository.createStatForAllPlayers('isActive', true);

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

        it('should throw ValidationError when creating a stat with an empty name', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => statsRepository.createStatForAllPlayers('', 'test')).toThrow(
                ValidationError,
            );
            expect(() => statsRepository.createStatForAllPlayers('', 'test')).toThrow(
                'Stat name cannot be empty.',
            );
        });

        it('should throw ValidationError when creating a stat with a whitespace-only name', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => statsRepository.createStatForAllPlayers('   ', 'test')).toThrow(
                ValidationError,
            );
            expect(() => statsRepository.createStatForAllPlayers('   ', 'test')).toThrow(
                'Stat name cannot be empty.',
            );
        });
    });

    describe('createStatForPlayer', () => {
        it('should create a stat for a specific player', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            statsRepository.createStatForPlayer(1, 'level', 'test');

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

            expect(() => statsRepository.createStatForPlayer(1, 'level', 10)).toThrow(Conflict);
            expect(() => statsRepository.createStatForPlayer(1, 'level', 10)).toThrow(
                'Player with ID 1 already has a stat named "level".',
            );
        });

        it('should throw NotFound if the player does not exist', () => {
            expect(() => statsRepository.createStatForPlayer(999, 'level', 5)).toThrow(NotFound);
            expect(() => statsRepository.createStatForPlayer(999, 'level', 5)).toThrow(
                'Player with ID 999 does not exist.',
            );
        });

        it('should allow creating stats with no value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            statsRepository.createStatForPlayer(1, 'level', '');

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

            statsRepository.createStatForPlayer(1, 'level', 5);

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

            statsRepository.createStatForPlayer(1, 'isActive', false);

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

        it('should throw ValidationError when creating a stat with an empty name', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => statsRepository.createStatForPlayer(1, '', 'test')).toThrow(
                ValidationError,
            );
            expect(() => statsRepository.createStatForPlayer(1, '', 'test')).toThrow(
                'Stat name cannot be empty.',
            );
        });

        it('should throw ValidationError when creating a stat with a whitespace-only name', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => statsRepository.createStatForPlayer(1, '   ', 'test')).toThrow(
                ValidationError,
            );
            expect(() => statsRepository.createStatForPlayer(1, '   ', 'test')).toThrow(
                'Stat name cannot be empty.',
            );
        });
    });

    describe('updateStatForPlayer', () => {
        it('should update a stat for a specific player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            statsRepository.updateStatForPlayer(1, 1, 'level', 10);

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

        it('should throw NotFound if the stat does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            expect(() => statsRepository.updateStatForPlayer(999, 999, 'level', 10)).toThrow(
                NotFound,
            );
            expect(() => statsRepository.updateStatForPlayer(999, 999, 'level', 10)).toThrow(
                'Stat with ID 999 does not exist for player with ID 999.',
            );
        });

        it('should throw NotFound if the stat exists but for a different player', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            expect(() => statsRepository.updateStatForPlayer(2, 1, 'level', 10)).toThrow(NotFound);
            expect(() => statsRepository.updateStatForPlayer(2, 1, 'level', 10)).toThrow(
                'Stat with ID 1 does not exist for player with ID 2.',
            );
        });

        it('should throw Conflict when updating to a duplicate stat name for the same player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5'), (2, 1, 'score', 'number', '100')",
            );

            expect(() => statsRepository.updateStatForPlayer(1, 2, 'level', 10)).toThrow(Conflict);
            expect(() => statsRepository.updateStatForPlayer(1, 2, 'level', 10)).toThrow(
                'Player with ID 1 already has a stat named "level".',
            );
        });

        it('should throw ValidationError when updating stat name to an empty string', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            expect(() => statsRepository.updateStatForPlayer(1, 1, '')).toThrow(ValidationError);
            expect(() => statsRepository.updateStatForPlayer(1, 1, '')).toThrow(
                'Stat name cannot be empty.',
            );
        });

        it('should allow updating the stat to have no value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, 'level', '');

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

        it('should throw ValidationError if no fields are provided to update', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            expect(() => statsRepository.updateStatForPlayer(1, 1)).toThrow(ValidationError);
            expect(() => statsRepository.updateStatForPlayer(1, 1)).toThrow(
                'No fields provided to update.',
            );
        });

        it('should update stats to a number value', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'string', 'test')",
            );

            statsRepository.updateStatForPlayer(1, 1, undefined, 42);

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

            statsRepository.updateStatForPlayer(1, 1, undefined, true);

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

            statsRepository.updateStatForPlayer(1, 1, undefined, 'expert');

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
        it('should throw NotFound if the stat does not exist for the player', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (id, player_id, name, type, value) VALUES (1, 1, 'level', 'number', '5')",
            );

            expect(() => statsRepository.removeStatFromPlayer(1, 999)).toThrow(NotFound);
            expect(() => statsRepository.removeStatFromPlayer(1, 999)).toThrow(
                'Stat with ID 999 does not exist for player with ID 1.',
            );
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

        it('should throw NotFound if the player does not exist', () => {
            expect(() => statsRepository.removeAllStatsFromPlayer(999)).toThrow(NotFound);
            expect(() => statsRepository.removeAllStatsFromPlayer(999)).toThrow(
                'Player with ID 999 does not exist.',
            );
        });
    });
});

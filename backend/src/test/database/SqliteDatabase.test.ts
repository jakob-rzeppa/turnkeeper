import { beforeEach, describe, expect, it, vi } from 'vitest';

import config from '../../config/config.js';
import { SqliteDatabase } from '../../database/SqliteDatabase.js';

vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('SqliteDatabase', () => {
    beforeEach(() => {
        vi.mocked(config).dbPath = ':memory:';
    });

    describe('constructor', () => {
        it('should throw an error if dbPath is not defined in the configuration', () => {
            vi.mocked(config).dbPath = '';

            expect(() => {
                SqliteDatabase.getInstance();
            }).toThrowError('Database path is not defined in the configuration.');
        });

        it('should create tables when instantiated with a valid dbPath', () => {
            const db = SqliteDatabase.getInstance();

            // Check if tables exist
            const tables = (
                db
                    .prepare(
                        "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('players', 'player_stats', 'game_state', 'tradables', 'player_tradables', 'messages')",
                    )
                    .all() as { name: string }[]
            ).map((row) => row.name);

            expect(tables).toContain('players');
            expect(tables).toContain('player_stats');
            expect(tables).toContain('game_state');
            expect(tables).toContain('tradables');
            expect(tables).toContain('player_tradables');
            expect(tables).toContain('messages');
        });
    });

    describe('dropTables', () => {
        it('should drop all tables', () => {
            const db = SqliteDatabase.getInstance();

            // Ensure tables exist before dropping
            let tables = (
                db
                    .prepare(
                        "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('players', 'player_stats', 'game_state', 'tradables', 'player_tradables', 'messages')",
                    )
                    .all() as { name: string }[]
            ).map((row) => row.name);

            expect(tables).toContain('players');
            expect(tables).toContain('player_stats');
            expect(tables).toContain('game_state');
            expect(tables).toContain('tradables');
            expect(tables).toContain('player_tradables');
            expect(tables).toContain('messages');

            // Drop tables
            db.dropTables();

            // Check if tables have been dropped
            tables = (
                db
                    .prepare(
                        "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('players', 'player_stats', 'game_state', 'tradables', 'player_tradables', 'messages')",
                    )
                    .all() as { name: string }[]
            ).map((row) => row.name);

            expect(tables).not.toContain('players');
            expect(tables).not.toContain('player_stats');
            expect(tables).not.toContain('game_state');
            expect(tables).not.toContain('tradables');
            expect(tables).not.toContain('player_tradables');
            expect(tables).not.toContain('messages');
        });
    });
});

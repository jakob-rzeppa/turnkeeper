import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase.js';
import { tradableRepository } from '../../repositories/tradableRepository.js';
import { Conflict, NotFound, ValidationError } from '../../repositories/repositoryErrors.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('Tradable Repository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('createTradable', () => {
        it('should create a new tradable with the given name', () => {
            const tradable = tradableRepository.createTradable('Gold');

            expect(tradable).toBeDefined();
            expect(tradable.name).toBe('Gold');
            expect(tradable.initialQuantity).toBe(0);
            expect(tradable.id).toBeDefined();

            // Verify it is stored in the database
            const storedTradable = db
                .prepare('SELECT * FROM tradables WHERE id = ?')
                .get(tradable.id);
            expect(storedTradable).toMatchObject({
                id: tradable.id,
                name: 'Gold',
                initial_quantity: 0,
            });
        });

        it('should create multiple tradables with different names', () => {
            const tradable1 = tradableRepository.createTradable('Gold');
            const tradable2 = tradableRepository.createTradable('Silver');

            expect(tradable1.name).toBe('Gold');
            expect(tradable2.name).toBe('Silver');
            expect(tradable1.id).not.toBe(tradable2.id);

            // Verify both are stored in the database
            const storedTradable1 = db
                .prepare('SELECT * FROM tradables WHERE id = ?')
                .get(tradable1.id);
            const storedTradable2 = db
                .prepare('SELECT * FROM tradables WHERE id = ?')
                .get(tradable2.id);
            expect(storedTradable1).toMatchObject({
                id: tradable1.id,
                name: 'Gold',
                initial_quantity: 0,
            });
            expect(storedTradable2).toMatchObject({
                id: tradable2.id,
                name: 'Silver',
                initial_quantity: 0,
            });
        });

        it('should throw a Conflict error if a tradable with the same name already exists', () => {
            tradableRepository.createTradable('Gold');

            expect(() => tradableRepository.createTradable('Gold')).toThrow(Conflict);

            // Verify only one tradable is stored in the database
            const tradables = db.prepare('SELECT * FROM tradables WHERE name = ?').all('Gold');
            expect(tradables).toHaveLength(1);
        });

        it('should throw an error if the name is empty', () => {
            expect(() => tradableRepository.createTradable('')).toThrow(ValidationError);

            // Verify no tradable is stored in the database
            const tradables = db.prepare('SELECT * FROM tradables').all();
            expect(tradables).toHaveLength(0);
        });
    });

    describe('initializeTradableForAllPlayers', () => {
        it('should initialize a tradable for all existing players', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create a tradable with initial quantity
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            tradableRepository.initializeTradableForAllPlayers(1);

            // Check player_tradables entries
            const aliceTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 1')
                .all();
            const bobTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 2 AND tradable_id = 1')
                .all();

            expect(aliceTradables).toHaveLength(1);
            expect(aliceTradables[0]).toMatchObject({
                player_id: 1,
                tradable_id: 1,
                quantity: 100,
            });
            expect(bobTradables).toHaveLength(1);
            expect(bobTradables[0]).toMatchObject({
                player_id: 2,
                tradable_id: 1,
                quantity: 100,
            });
        });

        it('should skip players who already have the tradable initialized', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create a tradable with initial quantity
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            // Alice already has this tradable with quantity 50
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (1, 1, 50)',
            );

            tradableRepository.initializeTradableForAllPlayers(1);

            // Alice's quantity should remain unchanged
            const aliceTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 1')
                .all();
            expect(aliceTradables).toHaveLength(1);
            expect(aliceTradables[0]).toMatchObject({
                player_id: 1,
                tradable_id: 1,
                quantity: 50, // Should not change
            });

            // Bob should be initialized with the default quantity
            const bobTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 2 AND tradable_id = 1')
                .all();
            expect(bobTradables).toHaveLength(1);
            expect(bobTradables[0]).toMatchObject({
                player_id: 2,
                tradable_id: 1,
                quantity: 100,
            });
        });

        it('should throw NotFound error if the tradable does not exist', () => {
            expect(() => tradableRepository.initializeTradableForAllPlayers(999)).toThrow(NotFound);
        });

        it('should do nothing if there are no players', () => {
            // Create a tradable
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            // Should not throw an error
            expect(() => tradableRepository.initializeTradableForAllPlayers(1)).not.toThrow();

            // Verify no player_tradables entries were created
            const tradables = db.prepare('SELECT * FROM player_tradables').all();
            expect(tradables).toHaveLength(0);
        });
    });

    describe('initializeTradableForPlayer', () => {
        it('should initialize a tradable for a specific player', () => {
            // Create a player
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Create a tradable with initial quantity
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            tradableRepository.initializeTradableForPlayer(1, 1);

            // Check player_tradables entry
            const playerTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 1')
                .all();

            expect(playerTradables).toHaveLength(1);
            expect(playerTradables[0]).toMatchObject({
                player_id: 1,
                tradable_id: 1,
                quantity: 100,
            });
        });

        it('should do nothing if the tradable is already initialized for the player', () => {
            // Create a player
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Create a tradable
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            // Player already has this tradable with quantity 50
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (1, 1, 50)',
            );

            tradableRepository.initializeTradableForPlayer(1, 1);

            // Quantity should remain unchanged
            const playerTradables = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 1')
                .all();

            expect(playerTradables).toHaveLength(1);
            expect(playerTradables[0]).toMatchObject({
                player_id: 1,
                tradable_id: 1,
                quantity: 50, // Should not change
            });
        });

        it('should throw NotFound error if the player does not exist', () => {
            // Create a tradable
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            expect(() => tradableRepository.initializeTradableForPlayer(999, 1)).toThrow(NotFound);

            // Verify no player_tradables entry was created
            const tradables = db.prepare('SELECT * FROM player_tradables').all();
            expect(tradables).toHaveLength(0);
        });

        it('should throw NotFound error if the tradable does not exist', () => {
            // Create a player
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => tradableRepository.initializeTradableForPlayer(1, 999)).toThrow(NotFound);

            // Verify no player_tradables entry was created
            const tradables = db.prepare('SELECT * FROM player_tradables').all();
            expect(tradables).toHaveLength(0);
        });

        it('should handle multiple tradables for the same player', () => {
            // Create a player
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Create multiple tradables
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (2, 'Silver', 200)");

            tradableRepository.initializeTradableForPlayer(1, 1);
            tradableRepository.initializeTradableForPlayer(1, 2);

            // Check both tradables are initialized
            const goldTradable = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 1')
                .get();
            const silverTradable = db
                .prepare('SELECT * FROM player_tradables WHERE player_id = 1 AND tradable_id = 2')
                .get();

            expect(goldTradable).toMatchObject({
                player_id: 1,
                tradable_id: 1,
                quantity: 100,
            });
            expect(silverTradable).toMatchObject({
                player_id: 1,
                tradable_id: 2,
                quantity: 200,
            });
        });
    });

    describe('deleteTradable', () => {
        it('should delete a tradable by its ID', () => {
            // Create a tradable
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            tradableRepository.deleteTradable(1);

            // Verify the tradable is deleted
            const tradables = db.prepare('SELECT * FROM tradables WHERE id = 1').all();
            expect(tradables).toHaveLength(0);
        });

        it('should delete all associated player_tradables entries', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create a tradable
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");

            // Create player_tradables entries
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (1, 1, 50)',
            );
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (2, 1, 75)',
            );

            tradableRepository.deleteTradable(1);

            // Verify the tradable is deleted
            const tradables = db.prepare('SELECT * FROM tradables WHERE id = 1').all();
            expect(tradables).toHaveLength(0);

            // Verify all player_tradables entries are deleted
            const playerTradables = db
                .prepare('SELECT * FROM player_tradables WHERE tradable_id = 1')
                .all();
            expect(playerTradables).toHaveLength(0);
        });

        it('should throw NotFound error if the tradable does not exist', () => {
            expect(() => tradableRepository.deleteTradable(999)).toThrow(NotFound);
        });

        it('should not affect other tradables', () => {
            // Create multiple tradables
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (2, 'Silver', 200)");

            tradableRepository.deleteTradable(1);

            // Verify Gold is deleted
            const goldTradables = db.prepare('SELECT * FROM tradables WHERE id = 1').all();
            expect(goldTradables).toHaveLength(0);

            // Verify Silver still exists
            const silverTradables = db.prepare('SELECT * FROM tradables WHERE id = 2').all();
            expect(silverTradables).toHaveLength(1);
            expect(silverTradables[0]).toMatchObject({
                id: 2,
                name: 'Silver',
                initial_quantity: 200,
            });
        });

        it('should not affect player_tradables entries for other tradables', () => {
            // Create a player
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Create multiple tradables
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (1, 'Gold', 100)");
            db.exec("INSERT INTO tradables (id, name, initial_quantity) VALUES (2, 'Silver', 200)");

            // Create player_tradables entries for both tradables
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (1, 1, 50)',
            );
            db.exec(
                'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (1, 2, 75)',
            );

            tradableRepository.deleteTradable(1);

            // Verify Gold player_tradables entry is deleted
            const goldPlayerTradables = db
                .prepare('SELECT * FROM player_tradables WHERE tradable_id = 1')
                .all();
            expect(goldPlayerTradables).toHaveLength(0);

            // Verify Silver player_tradables entry still exists
            const silverPlayerTradables = db
                .prepare('SELECT * FROM player_tradables WHERE tradable_id = 2')
                .all();
            expect(silverPlayerTradables).toHaveLength(1);
            expect(silverPlayerTradables[0]).toMatchObject({
                player_id: 1,
                tradable_id: 2,
                quantity: 75,
            });
        });
    });
});

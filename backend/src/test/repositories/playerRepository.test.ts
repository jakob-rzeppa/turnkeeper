import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase';
import playerRepository from '../../repositories/playerRepository';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

// This test checks the playerRepository functions.
describe('Player Repository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('getAllPlayers', () => {
        it('should return all players from the database', () => {
            db.prepare(
                "INSERT INTO players (name, secret) VALUES ('Alice', 'secret1'), ('Bob', 'secret2')",
            ).run();

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(2);
            expect(players[0].name).toBe('Alice');
            expect(players[0].secret).toBe('secret1');
            expect(players[1].name).toBe('Bob');
            expect(players[1].secret).toBe('secret2');
        });

        it('should return an empty array if no players exist', () => {
            const players = playerRepository.getAllPlayers();
            expect(players).toHaveLength(0);
        });

        it('should return players with their stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'score', '100'), (1, 'level', '5')",
            );

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].stats).toBeDefined();
            expect(players[0].stats).toHaveLength(2);
            expect(players[0].stats).toContainEqual({
                id: 1,
                name: 'score',
                value: '100',
            });
            expect(players[0].stats).toContainEqual({
                id: 2,
                name: 'level',
                value: '5',
            });
        });

        it('should return players with empty stats array if they have no stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].stats).toBeDefined();
            expect(players[0].stats).toHaveLength(0);
        });

        it('should return players with stats, that have empty values', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'score', ''), (1, 'level', '5')",
            );

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].stats).toBeDefined();
            expect(players[0].stats).toHaveLength(2);
            expect(players[0].stats).toContainEqual({
                id: 1,
                name: 'score',
                value: '',
            });
            expect(players[0].stats).toContainEqual({
                id: 2,
                name: 'level',
                value: '5',
            });
        });
    });

    describe('getPlayerById', () => {
        it('should return a player by id from the database', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.secret).toBe('secret1');
        });

        it('should return null if player does not exist', () => {
            const player = playerRepository.getPlayerById(999);
            expect(player).toBeNull();
        });

        it('should return players with their stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'score', '100'), (1, 'level', '5')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.stats).toHaveLength(2);
            expect(player?.stats).toContainEqual({
                id: 1,
                name: 'score',
                value: '100',
            });
            expect(player?.stats).toContainEqual({
                id: 2,
                name: 'level',
                value: '5',
            });
        });

        it('should return players with empty stats array if they have no stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.stats).toBeDefined();
            expect(player?.stats).toHaveLength(0);
        });

        it('should return players with stats, that have empty values', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, value) VALUES (1, 'score', ''), (1, 'level', '5')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.stats).toBeDefined();
            expect(player?.stats).toHaveLength(2);
            expect(player?.stats).toContainEqual({
                id: 1,
                name: 'score',
                value: '',
            });
            expect(player?.stats).toContainEqual({
                id: 2,
                name: 'level',
                value: '5',
            });
        });
    });

    describe('getPlayerIdByName', () => {
        it('should return a player id by name from the database', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            const playerId = playerRepository.getPlayerIdByName('Bob');
            expect(playerId).toBe(2);
        });

        it('should return null if player name does not exist', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            const playerId = playerRepository.getPlayerIdByName('non-existent-name');
            expect(playerId).toBeNull();
        });
    });
    describe('getPlayerNameById', () => {
        it('should return a player name by id from the database', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            const playerName = playerRepository.getPlayerNameById(2);
            expect(playerName).toBe('Bob');
        });

        it('should return null if player id does not exist', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            const playerName = playerRepository.getPlayerNameById(5);
            expect(playerName).toBeNull();
        });
    });
    describe('createPlayer', () => {
        it('should create a new player in the database', () => {
            playerRepository.createPlayer('Charlie');

            const player = db.prepare('SELECT * FROM players WHERE name = ?').get('Charlie') as {
                id: number;
                name: string;
                secret: string;
            };

            expect(player.name).toBe('Charlie');
            expect(player.secret).toHaveLength(4); // The secret length is 4
        });
        it('should not create a player with a duplicate name', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Charlie', 'secret1')");

            playerRepository.createPlayer('Charlie');

            const players = db
                .prepare('SELECT * FROM players WHERE name = ?')
                .all('Charlie') as object[];

            expect(players).toHaveLength(1); // Only one player with the name "Charlie" should exist
        });
    });
    describe('updatePlayer', () => {
        it("should update an existing player's name in the database", () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            playerRepository.updatePlayer(1, { name: 'AliceUpdated' });

            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(1) as {
                id: number;
                name: string;
                secret: string;
            };

            expect(player.name).toBe('AliceUpdated');
        });

        it('should not update a non-existent player', () => {
            playerRepository.updatePlayer(999, { name: 'NonExistent' });

            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(999) as
                | undefined
                | { id: number; name: string; secret: string };

            expect(player).toBeUndefined();
        });

        it("should not update a player's name to a duplicate name", () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );

            playerRepository.updatePlayer(1, { name: 'Bob' });
            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(1) as {
                id: number;
                name: string;
                secret: string;
            };

            expect(player.name).toBe('Alice'); // Name should remain unchanged
        });

        it('should not update if there are no fields to update supplied', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            playerRepository.updatePlayer(1, {});
            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(1) as {
                id: number;
                name: string;
                secret: string;
            };

            expect(player.name).toBe('Alice'); // Name should remain unchanged
        });
    });
    describe('deletePlayer', () => {
        it('should delete an existing player from the database', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            playerRepository.deletePlayer(1);

            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(1) as
                | undefined
                | { id: number; name: string; secret: string };

            expect(player).toBeUndefined();
        });

        it('should not fail when trying to delete a non-existent player', () => {
            expect(() => {
                playerRepository.deletePlayer(999);
            }).not.toThrow();
        });
    });
});

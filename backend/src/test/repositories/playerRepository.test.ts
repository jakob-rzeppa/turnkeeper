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
                "INSERT INTO players (name, secret, notes, hidden_notes) VALUES ('Alice', 'secret1', 'notes1', 'hidden1'), ('Bob', 'secret2', 'notes2', 'hidden2')",
            ).run();

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(2);
            expect(players[0].name).toBe('Alice');
            expect(players[0].secret).toBe('secret1');
            expect(players[0].notes).toBe('notes1');
            expect(players[0].hiddenNotes).toBe('hidden1');
            expect(players[1].name).toBe('Bob');
            expect(players[1].secret).toBe('secret2');
            expect(players[1].notes).toBe('notes2');
            expect(players[1].hiddenNotes).toBe('hidden2');
        });

        it('should return an empty array if no players exist', () => {
            const players = playerRepository.getAllPlayers();
            expect(players).toHaveLength(0);
        });

        it('should return players with their stats', () => {
            db.exec(
                "INSERT INTO players (id, name, secret, notes) VALUES (1, 'Alice', 'secret1', 'notes1')",
            );
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', 'high'), (1, 'level', 'string', 'low')",
            );

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].stats).toBeDefined();
            expect(players[0].stats).toHaveLength(2);
            expect(players[0].stats).toContainEqual({
                id: 1,
                name: 'score',
                value: 'high',
            });
            expect(players[0].stats).toContainEqual({
                id: 2,
                name: 'level',
                value: 'low',
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
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', ''), (1, 'level', 'string', 'high')",
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
                value: 'high',
            });
        });

        it('should return multiple players with their stats', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', 'high'), (2, 'level', 'string', 'low')",
            );

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(2);

            const alice = players.find((p) => p.name === 'Alice');
            const bob = players.find((p) => p.name === 'Bob');

            expect(alice).toBeDefined();
            expect(alice?.stats).toHaveLength(1);
            expect(alice?.stats).toContainEqual({
                id: 1,
                name: 'score',
                value: 'high',
            });

            expect(bob).toBeDefined();
            expect(bob?.stats).toHaveLength(1);
            expect(bob?.stats).toContainEqual({
                id: 2,
                name: 'level',
                value: 'low',
            });
        });

        it('should return player stats with different types and values', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'isActive', 'boolean', 'true'), (1, 'score', 'number', '42'), (1, 'rank', 'string', 'gold')",
            );

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].stats).toBeDefined();
            expect(players[0].stats).toHaveLength(3);
            expect(players[0].stats).toContainEqual({
                id: 1,
                name: 'isActive',
                value: true,
            });
            expect(players[0].stats).toContainEqual({
                id: 2,
                name: 'score',
                value: 42,
            });
            expect(players[0].stats).toContainEqual({
                id: 3,
                name: 'rank',
                value: 'gold',
            });
        });

        it('should return players with empty notes', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].notes).toBe(''); // Default value for notes should be an empty string
        });

        it('should return players with empty hidden notes', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const players = playerRepository.getAllPlayers();

            expect(players).toHaveLength(1);
            expect(players[0].name).toBe('Alice');
            expect(players[0].hiddenNotes).toBe(''); // Default value for hiddenNotes should be an empty string
        });
    });

    describe('getPlayerById', () => {
        it('should return a player by id from the database', () => {
            db.exec(
                "INSERT INTO players (id, name, secret, notes, hidden_notes) VALUES (1, 'Alice', 'secret1', 'notes1', 'hidden1'), (2, 'Bob', 'secret2', 'notes2', 'hidden2')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.secret).toBe('secret1');
            expect(player?.notes).toBe('notes1');
            expect(player?.hiddenNotes).toBe('hidden1');
        });

        it('should return null if player does not exist', () => {
            const player = playerRepository.getPlayerById(999);
            expect(player).toBeNull();
        });

        it('should return players with their stats', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', 'high'), (1, 'level', 'string', 'low')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.stats).toHaveLength(2);
            expect(player?.stats).toContainEqual({
                id: 1,
                name: 'score',
                value: 'high',
            });
            expect(player?.stats).toContainEqual({
                id: 2,
                name: 'level',
                value: 'low',
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
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'score', 'string', ''), (1, 'level', 'string', 'high')",
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
                value: 'high',
            });
        });

        it('should return player stats with different types and values', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO player_stats (player_id, name, type, value) VALUES (1, 'isActive', 'boolean', 'true'), (1, 'score', 'number', '42'), (1, 'rank', 'string', 'gold')",
            );

            const player = playerRepository.getPlayerById(1);

            expect(player).toBeDefined();
            expect(player?.name).toBe('Alice');
            expect(player?.stats).toBeDefined();
            expect(player?.stats).toHaveLength(3);
            expect(player?.stats).toContainEqual({
                id: 1,
                name: 'isActive',
                value: true,
            });
            expect(player?.stats).toContainEqual({
                id: 2,
                name: 'score',
                value: 42,
            });
            expect(player?.stats).toContainEqual({
                id: 3,
                name: 'rank',
                value: 'gold',
            });
        });

        it('should return players with empty notes', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const player = playerRepository.getPlayerById(1);

            expect(player).not.toBeNull();

            if (!player) return;

            expect(player.name).toBe('Alice');
            expect(player.notes).toBe(''); // Default value for notes should be an empty string
            expect(player.hiddenNotes).toBe(''); // Default value for hiddenNotes should be an empty string
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
                notes?: string;
                secret: string;
            };

            expect(player.name).toBe('Charlie');
            expect(player.secret).toHaveLength(4); // The secret length is 4
            expect(player.notes).toBe(''); // Default value for notes should be an empty string
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

        it("should update an existing player's secret in the database", () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            playerRepository.updatePlayer(1, { secret: 'newSecret' });

            const player = db.prepare('SELECT * FROM players WHERE id = ?').get(1) as {
                id: number;
                name: string;
                secret: string;
            };

            expect(player.secret).toBe('newSecret');
        });

        it("should update an existing player's notes in the database", () => {
            db.exec(
                "INSERT INTO players (id, name, secret, notes) VALUES (1, 'Alice', 'secret1', '')",
            );

            playerRepository.updatePlayer(1, { notes: 'Updated notes' });

            const player = db.prepare('SELECT notes FROM players WHERE id = ?').get(1) as {
                notes: string;
            };

            expect(player.notes).toBe('Updated notes');
        });

        it("should update an existing player's notes in the database", () => {
            db.exec(
                "INSERT INTO players (id, name, secret, notes, hidden_notes) VALUES (1, 'Alice', 'secret1', '', '')",
            );

            playerRepository.updatePlayer(1, { hiddenNotes: 'Updated hidden notes' });

            const player = db
                .prepare('SELECT hidden_notes AS hiddenNotes FROM players WHERE id = ?')
                .get(1) as {
                hiddenNotes: string;
            };

            expect(player.hiddenNotes).toBe('Updated hidden notes');
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

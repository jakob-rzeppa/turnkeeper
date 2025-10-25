import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase';
import gameStateRepository from '../../repositories/gameStateRepository';
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
        info: vi.fn(),
        warn: vi.fn(),
    },
}));

describe('gameStateRepository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('getGameStateById', () => {
        it('should return a game state object', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2'), (3, 'Charlie', 'secret3')",
            );
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 2, 0, '1,2,3')
            `);

            const gamestate = gameStateRepository.getGameStateById(1);

            expect(gamestate).toEqual({
                currentPlayerIndex: 0,
                id: 1,
                playerOrder: [
                    { id: 1, name: 'Alice' },
                    { id: 2, name: 'Bob' },
                    { id: 3, name: 'Charlie' },
                ],
                roundNumber: 2,
            });
        });

        it('should return undefined for non-existing game state', () => {
            const gamestate = gameStateRepository.getGameStateById(999);
            expect(gamestate).toBeNull();
        });

        it('should handle empty player order', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (2, 1, 0, '')
            `);

            const gamestate = gameStateRepository.getGameStateById(2);

            expect(gamestate).toEqual({
                currentPlayerIndex: 0,
                id: 2,
                playerOrder: [],
                roundNumber: 1,
            });
        });

        it('should handle single player in player order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (4, 'Dave', 'secret4')");
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (3, 1, 0, '4')
            `);

            const gamestate = gameStateRepository.getGameStateById(3);

            expect(gamestate).toEqual({
                currentPlayerIndex: 0,
                id: 3,
                playerOrder: [{ id: 4, name: 'Dave' }],
                roundNumber: 1,
            });
        });

        it('should return null for inconsistent game state', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (5, 1, 0, '10,11')
            `);

            const gamestate = gameStateRepository.getGameStateById(5);

            expect(gamestate).toBeNull();
            expect(logger.error).toHaveBeenCalledWith({
                message:
                    'Inconsistent game state: some player IDs in the game state do not exist in the players table.',
            });
        });

        it('should have the correct order of players', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (5, 'Eve', 'secret5'), (6, 'Frank', 'secret6'), (7, 'Grace', 'secret7')",
            );
            db.exec(`
                INSERT INTO game_state (round_number, current_player_index, player_order)
                VALUES (3, 1, '7,5,6')
            `);

            const gamestate = gameStateRepository.getGameStateById(1);

            expect(gamestate).toEqual({
                currentPlayerIndex: 1,
                id: 1,
                playerOrder: [
                    { id: 7, name: 'Grace' },
                    { id: 5, name: 'Eve' },
                    { id: 6, name: 'Frank' },
                ],
                roundNumber: 3,
            });
        });
    });

    describe('createGameState', () => {
        it('should create a new game state', () => {
            const newGameState = {
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: 'Alice' },
                    { id: 2, name: 'Bob' },
                ],
                roundNumber: 0,
            };

            gameStateRepository.createGameState(newGameState);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                current_player_index: number;
                id: number;
                player_order: string;
                round_number: number;
            };

            expect(row).toBeDefined();
            expect(row.round_number).toBe(0);
            expect(row.current_player_index).toBe(0);
            expect(row.player_order).toBe('1,2');
        });
    });

    describe('updateGameState', () => {
        describe('when updating an existing game state', () => {
            it('should update the game state', () => {
                db.exec(`
                    INSERT INTO game_state (id, round_number, current_player_index, player_order)
                    VALUES (1, 1, 0, '1,2')
                `);

                gameStateRepository.updateGameState(1, {
                    currentPlayerIndex: 1,
                    roundNumber: 2,
                });

                const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                    current_player_index: number;
                    id: number;
                    player_order: string;
                    round_number: number;
                };

                expect(row.round_number).toBe(2);
                expect(row.current_player_index).toBe(1);
            });

            it('should not update non-provided fields', () => {
                db.exec(`
                    INSERT INTO game_state (id, round_number, current_player_index, player_order)
                    VALUES (1, 1, 0, '1,2')
                `);

                gameStateRepository.updateGameState(1, {
                    roundNumber: 3,
                });

                const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                    current_player_index: number;
                    id: number;
                    player_order: string;
                    round_number: number;
                };

                expect(row.round_number).toBe(3);
                expect(row.current_player_index).toBe(0);
            });

            it('should update player order', () => {
                db.exec(`
                    INSERT INTO game_state (id, round_number, current_player_index, player_order)
                    VALUES (1, 1, 0, '1,2')
                `);

                gameStateRepository.updateGameState(1, {
                    playerOrder: [
                        { id: 2, name: 'Bob' },
                        { id: 1, name: 'Alice' },
                    ],
                });

                const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                    current_player_index: number;
                    id: number;
                    player_order: string;
                    round_number: number;
                };

                expect(row.player_order).toBe('2,1');
            });
        });
    });

    describe('deleteGameState', () => {
        it('should delete the game state', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 1, 0, '1,2')
            `);

            gameStateRepository.deleteGameState(1);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get();

            expect(row).toBeUndefined();
        });
    });

    describe('removeDeletedPlayersFromPlayerOrder', () => {
        it('should remove deleted players from the player order', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 1, 0, '1,2,3')
            `);

            gameStateRepository.removeDeletedPlayersFromPlayerOrder([1, 3]);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                current_player_index: number;
                id: number;
                player_order: string;
                round_number: number;
            };

            expect(row.player_order).toBe('1,3');
        });

        it('should handle all players being deleted', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 1, 0, '1,2')
            `);

            gameStateRepository.removeDeletedPlayersFromPlayerOrder([]);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                current_player_index: number;
                id: number;
                player_order: string;
                round_number: number;
            };

            expect(row.player_order).toBe('');
        });

        it('should not change player order if all players exist', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 1, 0, '1,2')
            `);

            gameStateRepository.removeDeletedPlayersFromPlayerOrder([1, 2]);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                current_player_index: number;
                id: number;
                player_order: string;
                round_number: number;
            };

            expect(row.player_order).toBe('1,2');
        });

        it('should set currentPlayerIndex to 0 if it exceeds new player order length', () => {
            db.exec(`
                INSERT INTO game_state (id, round_number, current_player_index, player_order)
                VALUES (1, 1, 2, '1,2,3')
            `);

            gameStateRepository.removeDeletedPlayersFromPlayerOrder([1]);

            const row = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as {
                current_player_index: number;
                id: number;
                player_order: string;
                round_number: number;
            };

            expect(row.player_order).toBe('1');
            expect(row.round_number).toBe(2);
            expect(row.current_player_index).toBe(0);
        });
    });
});

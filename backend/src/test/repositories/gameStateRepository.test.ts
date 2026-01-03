import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase.js';
import gameStateRepository from '../../repositories/gameStateRepository.js';
import { Conflict, NotFound, ValidationError } from '../../repositories/repositoryErrors.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('gameStateRepository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('createGameState', () => {
        it('should create a new game state with valid player order', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            gameStateRepository.createGameState([1, 2]);

            // Verify game state is created
            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as any;
            expect(gameState).toBeDefined();
            expect(gameState.current_player_index).toBe(0);
            expect(gameState.round_number).toBe(1);

            // Verify player order
            const playerOrder = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];
            expect(playerOrder.map((row) => row.player_id)).toEqual([1, 2]);
        });

        it('should throw Conflict if game state already exists', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            gameStateRepository.createGameState([1, 2]);

            expect(() => gameStateRepository.createGameState([1, 2])).toThrow(Conflict);

            // Verify only one game state exists
            const gameStateCount = db.prepare('SELECT COUNT(*) as count FROM game_state').get() as {
                count: number;
            };
            expect(gameStateCount.count).toBe(1);
        });

        it('should throw ValidationError if player order contains invalid player IDs', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => gameStateRepository.createGameState([1, 999])).toThrow(ValidationError);

            // Verify no game state was created
            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get();
            expect(gameState).toBeUndefined();
        });

        it('should throw ValidationError if player order contains duplicate IDs', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => gameStateRepository.createGameState([1, 1])).toThrow(ValidationError);

            // Verify no game state was created
            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get();
            expect(gameState).toBeUndefined();
        });
    });

    describe('getGameStateById', () => {
        it('should return the game state by ID', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, 'test', 'secret')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            const gameState = gameStateRepository.getGameStateById(1);

            expect(gameState).toBeDefined();
            expect(gameState.id).toBe(1);
            expect(gameState.playerOrder).toEqual([1, 2]);
            expect(gameState.currentPlayerIndex).toBe(0);
            expect(gameState.roundNumber).toBe(0);
            expect(gameState.notes).toBe('test');
            expect(gameState.hiddenNotes).toBe('secret');
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.getGameStateById(999)).toThrow(NotFound);
        });
    });

    describe('updatePlayerOrder', () => {
        it('should update the player order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (3, 'Charlie', 'secret3')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1), (1, 3, 2)',
            );

            gameStateRepository.updatePlayerOrder(1, [2, 1, 3]);

            // Verify the order is updated in the database
            const gameStateRows = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];
            expect(gameStateRows.map((row) => row.player_id)).toEqual([2, 1, 3]);
        });

        it('should throw NotFound if game state does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => gameStateRepository.updatePlayerOrder(999, [1])).toThrow(NotFound);
        });

        it('should throw ValidationError if player order contains invalid player IDs', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );

            expect(() => gameStateRepository.updatePlayerOrder(1, [1, 999])).toThrow(
                ValidationError,
            );
        });

        it('should throw ValidationError if player order contains duplicates', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            gameStateRepository.createGameState([1]);

            expect(() => gameStateRepository.updatePlayerOrder(1, [1, 1])).toThrow(ValidationError);
        });
    });

    describe('addPlayerToOrder', () => {
        it('should add a player to the turn order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0)',
            );

            gameStateRepository.addPlayerToOrder(1, 2);

            const playerOrderRows = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];

            expect(playerOrderRows.map((row) => row.player_id)).toEqual([1, 2]);
        });

        it('should throw NotFound if game state does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => gameStateRepository.addPlayerToOrder(999, 1)).toThrow(NotFound);
        });

        it('should throw ValidationError if player ID does not exist', () => {
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );

            expect(() => gameStateRepository.addPlayerToOrder(1, 999)).toThrow(ValidationError);
        });

        it('should throw ValidationError if player is already in the order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            gameStateRepository.createGameState([1]);

            expect(() => gameStateRepository.addPlayerToOrder(1, 1)).toThrow(ValidationError);
        });
    });

    describe('removePlayerFromOrder', () => {
        it('should remove a player from the turn order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (3, 'Charlie', 'secret3')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1), (1, 3, 2)',
            );

            gameStateRepository.removePlayerFromOrder(1, 2);

            const playerOrderRows = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];

            expect(playerOrderRows.map((row) => row.player_id)).toEqual([1, 3]);
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.removePlayerFromOrder(999, 1)).toThrow(NotFound);

            // Verify no changes in player_order table
            const playerOrderRows = db.prepare('SELECT * FROM player_order').all() as {
                player_id: number;
            }[];

            expect(playerOrderRows).toEqual([]);
        });

        it('should throw NotFound if player is not in the order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            gameStateRepository.createGameState([1, 2]);

            expect(() => gameStateRepository.removePlayerFromOrder(1, 999)).toThrow(NotFound);

            // Verify no changes in player_order table
            const playerOrderRows = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];
            expect(playerOrderRows.map((row) => row.player_id)).toEqual([1, 2]);
        });

        it('should adjust current player index if removing player before current', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (3, 'Charlie', 'secret3')");
            gameStateRepository.createGameState([1, 2, 3]);

            // Advance to player 2 (index 1)
            gameStateRepository.advanceTurn(1);

            gameStateRepository.removePlayerFromOrder(1, 1);

            const playerOrderRows = db
                .prepare(
                    'SELECT player_id FROM player_order WHERE game_state_id = 1 ORDER BY position ASC',
                )
                .all() as { player_id: number }[];

            expect(playerOrderRows.map((row) => row.player_id)).toEqual([2, 3]);
        });
    });

    describe('advanceToNextPlayer', () => {
        it('should advance to the next player in turn order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            gameStateRepository.advanceTurn(1);

            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as any;
            expect(gameState.current_player_index).toBe(1);
            expect(gameState.round_number).toBe(0);
        });

        it('should loop back to first player and increment round number', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            // Advance to player 2
            gameStateRepository.advanceTurn(1);
            // Loop back to player 1, increment round
            gameStateRepository.advanceTurn(1);

            const gameStateRows = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as any;
            expect(gameStateRows.current_player_index).toBe(0);
            expect(gameStateRows.round_number).toBe(1);
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.advanceTurn(999)).toThrow(NotFound);
        });
    });

    describe('revertTurn', () => {
        it('should revert to the previous player in turn order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 1, 1, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            gameStateRepository.revertTurn(1);

            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as any;
            expect(gameState.current_player_index).toBe(0);
            expect(gameState.round_number).toBe(1);
        });

        it('should loop back to last player and decrement round number', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 2, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            gameStateRepository.revertTurn(1);

            const gameStateRows = db.prepare('SELECT * FROM game_state WHERE id = 1').get() as any;
            expect(gameStateRows.current_player_index).toBe(1);
            expect(gameStateRows.round_number).toBe(1);
        });
    });

    describe('updateNotes', () => {
        it('should update the public notes of the game state', () => {
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );

            gameStateRepository.updateNotes(1, 'New public notes');

            const gameState = gameStateRepository.getGameStateById(1);
            expect(gameState.notes).toBe('New public notes');
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.updateNotes(999, 'Notes')).toThrow(NotFound);
        });
    });

    describe('updateHiddenNotes', () => {
        it('should update the hidden notes of the game state', () => {
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );

            gameStateRepository.updateHiddenNotes(1, 'New hidden notes');

            const gameState = gameStateRepository.getGameStateById(1);
            expect(gameState.hiddenNotes).toBe('New hidden notes');
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.updateHiddenNotes(999, 'Hidden Notes')).toThrow(
                NotFound,
            );
        });
    });

    describe('deleteGameState', () => {
        it('should delete the game state', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO game_state (id, current_player_index, round_number, notes, hidden_notes) VALUES (1, 0, 0, '', '')",
            );
            db.exec(
                'INSERT INTO player_order (game_state_id, player_id, position) VALUES (1, 1, 0), (1, 2, 1)',
            );

            gameStateRepository.deleteGameState(1);

            const gameState = db.prepare('SELECT * FROM game_state WHERE id = 1').get();
            expect(gameState).toBeUndefined();

            // Verify player_order entries are also deleted
            const playerOrderRows = db
                .prepare('SELECT * FROM player_order WHERE game_state_id = 1')
                .all() as { player_id: number }[];
            expect(playerOrderRows).toEqual([]);
        });

        it('should throw NotFound if game state does not exist', () => {
            expect(() => gameStateRepository.deleteGameState(999)).toThrow(NotFound);
        });
    });
});

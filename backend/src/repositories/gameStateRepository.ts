import { GameState } from '../entities/GameState.js';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import { Conflict, DatabaseError, NotFound, ValidationError } from './repositoryErrors.js';

const db = SqliteDatabase.getInstance();

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;
// Later we might want to support multiple game states (e.g., for multiple concurrent games)
// In that case we would need to save the current game state Id in-memory

const gameStateRepository = {
    /**
     * Gets the game state by its ID
     *
     * @param id of the game state to retrieve
     * @returns the game state
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getGameStateById: (id: number): GameState => {
        const stmt = db.prepare(
            `SELECT gs.id, gs.current_player_index, gs.round_number, gs.notes, gs.hidden_notes, po.player_id AS player_id
            FROM game_state gs
            LEFT JOIN player_order po ON gs.id = po.game_state_id
            WHERE gs.id = ?
            ORDER BY po.position ASC`,
        );

        try {
            const row = stmt.all(id) as {
                id: number;
                current_player_index: number;
                round_number: number;
                notes: string;
                hidden_notes: string;
                player_id: number; // from player_order
            }[];

            if (row.length === 0) {
                throw new NotFound(`Game state with ID ${id} does not exist.`);
            }

            let playerOrder: number[] = [];
            for (const r of row) {
                playerOrder.push(r.player_id);
            }

            return {
                id: row[0].id,
                playerOrder,
                currentPlayerIndex: row[0].current_player_index,
                roundNumber: row[0].round_number,
                notes: row[0].notes,
                hiddenNotes: row[0].hidden_notes,
            };
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error retrieving game state.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error retrieving game state.');
        }
    },

    /**
     * Creates a new game state with the given fields
     *
     * @param player_order array of player IDs representing the turn order
     * @returns the created game state
     * @throws Conflict if a game state with the same ID already exists
     * @throws ValidationError if the player order contains invalid player IDs or not unique IDs
     * @throws DatabaseError if there was an unexpected error creating the game state
     */
    createGameState: (player_order: number[]): GameState => {
        // Validate player order before transaction
        if (player_order.length === 0) {
            throw new ValidationError('Player order cannot be empty.');
        }

        // Check for duplicate player IDs
        const uniqueIds = new Set(player_order);
        if (uniqueIds.size !== player_order.length) {
            throw new ValidationError('Player order contains duplicate player IDs.');
        }

        try {
            // Use a transaction to ensure both game_state and player_order entries are created atomically
            return db.transaction(() => {
                const stmt = db.prepare(
                    `INSERT INTO game_state (id, round_number, current_player_index, notes, hidden_notes)
                VALUES (${GAME_STATE_ID}, 1, 0, '', '')
                RETURNING id, round_number, current_player_index, notes, hidden_notes`,
                );

                const row = stmt.get() as {
                    id: number;
                    round_number: number;
                    current_player_index: number;
                    notes: string;
                    hidden_notes: string;
                };

                const gameStateId = row.id;

                // Insert player order entries
                const insertOrderStmt = db.prepare(
                    `INSERT INTO player_order (game_state_id, player_id, position)
                VALUES (?, ?, ?)`,
                );

                for (let i = 0; i < player_order.length; i++) {
                    insertOrderStmt.run(gameStateId, player_order[i], i);
                }

                return {
                    id: row.id,
                    playerOrder: player_order,
                    currentPlayerIndex: row.current_player_index,
                    roundNumber: row.round_number,
                    notes: row.notes,
                    hiddenNotes: row.hidden_notes,
                };
            })();
        } catch (err: any) {
            // Check SQLite error codes
            if (
                err.code === 'SQLITE_CONSTRAINT_PRIMARYKEY' ||
                err.code === 'SQLITE_CONSTRAINT_UNIQUE'
            )
                throw new Conflict(`Game state with ID ${GAME_STATE_ID} already exists.`);

            if (err.code === 'SQLITE_CONSTRAINT_FOREIGNKEY')
                throw new ValidationError('One or more player IDs in player order do not exist.');

            throw new DatabaseError('Unexpected error creating game state.');
        }
    },

    /**
     * Updates the player order of the game state
     *
     * @param game_state_id of the game state to update
     * @param player_order array of player IDs representing the new turn order
     * @throws NotFound if the game state does not exist
     * @throws ValidationError if the player order contains invalid player IDs or not unique IDs
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updatePlayerOrder: (game_state_id: number, player_order: number[]) => {
        try {
            // Check if game state exists
            const gameStateExists = db
                .prepare('SELECT 1 FROM game_state WHERE id = ?')
                .get(game_state_id);

            if (!gameStateExists) {
                throw new NotFound(`Game state with ID ${game_state_id} does not exist.`);
            }

            // Use transaction for atomic delete + insert
            db.transaction(() => {
                const deleteStmt = db.prepare('DELETE FROM player_order WHERE game_state_id = ?');
                deleteStmt.run(game_state_id);

                // Insert new player order entries
                const insertOrderStmt = db.prepare(
                    `INSERT INTO player_order (game_state_id, player_id, position)
            VALUES (?, ?, ?)`,
                );

                for (let i = 0; i < player_order.length; i++) {
                    insertOrderStmt.run(game_state_id, player_order[i], i);
                }
            })();
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error updating player order.');

            if (err instanceof NotFound) throw err;

            if (err.message.includes('FOREIGN KEY constraint failed'))
                throw new ValidationError('One or more player IDs in player order do not exist.');

            if (err.message.includes('UNIQUE constraint failed'))
                throw new ValidationError('Player order contains duplicate player IDs.');

            throw new DatabaseError('Unexpected error updating player order.');
        }
    },

    /**
     * Removes a player from the turn order of the game state
     *
     * @param game_state_id the ID of the game state to update
     * @param player_id the ID of the player to remove from the order
     * @throws NotFound if the game state or player in player_order does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    removePlayerFromOrder: (game_state_id: number, player_id: number) => {
        try {
            // Check if game state exists
            const gameStateExists = db
                .prepare('SELECT 1 FROM game_state WHERE id = ?')
                .get(game_state_id);

            if (!gameStateExists) {
                throw new NotFound(`Game state with ID ${game_state_id} does not exist.`);
            }

            const deleteStmt = db.prepare(
                'DELETE FROM player_order WHERE game_state_id = ? AND player_id = ?',
            );
            const result = deleteStmt.run(game_state_id, player_id);

            if (result.changes === 0) {
                throw new NotFound(
                    `Player with ID ${player_id} not found in player order for game state ${game_state_id}.`,
                );
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error removing player from order.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error removing player from order.');
        }
    },

    /**
     * Advances the game state to the next player's turn.
     *
     * Makes sure to loop back to the first player after the last player
     * and increments the round number accordingly.
     *
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    advanceToNextPlayer: (game_state_id: number) => {
        try {
            // Check if game state exists
            const gameState = db
                .prepare(
                    `SELECT gs.current_player_index, COUNT(po.player_id) AS player_count
                    FROM game_state gs
                    JOIN player_order po ON gs.id = po.game_state_id
                    WHERE gs.id = ?
                    GROUP BY gs.id`,
                )
                .get(game_state_id) as
                | { current_player_index: number; player_count: number }
                | undefined;

            if (!gameState) {
                throw new NotFound(`Game state with ID ${game_state_id} does not exist.`);
            }

            let newPlayerIndex = gameState.current_player_index + 1;

            if (newPlayerIndex >= gameState.player_count) {
                newPlayerIndex = 0;
            }

            const updateStmt = db.prepare(
                `UPDATE game_state
                SET current_player_index = ?, round_number = round_number + 1
                WHERE id = ?`,
            );

            updateStmt.run(newPlayerIndex, game_state_id);
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error advancing to next player.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error advancing to next player.');
        }
    },

    /**
     * Updates the notes of the game state.
     *
     * @param game_state_id the game state ID
     * @param notes the new public notes
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updateNotes: (game_state_id: number, notes: string) => {
        try {
            const updateStmt = db.prepare(
                `UPDATE game_state
                SET notes = ?
                WHERE id = ?`,
            );

            const result = updateStmt.run(notes, game_state_id);

            if (result.changes === 0) {
                throw new NotFound(`Game state with ID ${game_state_id} does not exist.`);
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error updating notes.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error updating notes.');
        }
    },

    /**
     * Updates the hidden notes of the game state.
     *
     * @param game_state_id the game state ID
     * @param hiddenNotes the new hidden notes
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updateHiddenNotes: (game_state_id: number, hiddenNotes: string) => {
        try {
            const updateStmt = db.prepare(
                `UPDATE game_state
                SET hidden_notes = ?
                WHERE id = ?`,
            );

            const result = updateStmt.run(hiddenNotes, game_state_id);

            if (result.changes === 0) {
                throw new NotFound(`Game state with ID ${game_state_id} does not exist.`);
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error updating hidden notes.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error updating hidden notes.');
        }
    },

    /**
     * Deletes the game state with the given ID.
     * It also deletes associated player order entries.
     *
     * @param id of the game state to delete
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error deleting the game state
     */
    deleteGameState: (id: number) => {
        try {
            // Use transaction to delete both game_state and player_order entries atomically
            db.transaction(() => {
                // First check if game state exists
                const gameStateExists = db.prepare('SELECT 1 FROM game_state WHERE id = ?').get(id);

                if (!gameStateExists) {
                    throw new NotFound(`Game state with ID ${id} does not exist.`);
                }

                // Delete player_order entries first (due to foreign key constraint)
                const deleteOrderStmt = db.prepare(
                    'DELETE FROM player_order WHERE game_state_id = ?',
                );
                deleteOrderStmt.run(id);

                // Delete game_state
                const deleteGameStateStmt = db.prepare('DELETE FROM game_state WHERE id = ?');
                deleteGameStateStmt.run(id);
            })();
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error deleting game state.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error deleting game state.');
        }
    },
};

export default gameStateRepository;

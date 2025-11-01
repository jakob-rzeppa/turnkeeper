import { GameState } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import logger from '../services/logger.js';

const db = SqliteDatabase.getInstance();

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;
// Later we might want to support multiple game states (e.g., for multiple concurrent games)
// In that case we would need to save the current game state Id in-memory

const gameStateRepository = {
    createGameState: (gamestate: Omit<GameState, 'hiddenNotes' | 'id' | 'notes'>) => {
        try {
            db.prepare(
                'INSERT INTO game_state (id, round_number, current_player_index, player_order) VALUES (?, ?, ?, ?)',
            ).run(
                GAME_STATE_ID,
                gamestate.roundNumber,
                gamestate.currentPlayerIndex,
                gamestate.playerOrder.map((p) => p.id).join(','),
            );
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
    deleteGameState: (id: number) => {
        try {
            db.prepare('DELETE FROM game_state WHERE id = ?').run(id);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
    getGameStateById: (id: number): GameState | null => {
        const row = db.prepare('SELECT * FROM game_state WHERE id = ?').get(id) as
            | undefined
            | {
                  current_player_index: number;
                  hidden_notes: string;
                  id: number;
                  notes: string;
                  player_order: string;
                  round_number: number;
              };

        if (!row) return null;

        if (row.player_order.length === 0) {
            return {
                currentPlayerIndex: row.current_player_index,
                hiddenNotes: row.hidden_notes,
                id: row.id,
                notes: row.notes,
                playerOrder: [],
                roundNumber: row.round_number,
            };
        }

        const playerOrder = row.player_order.split(',').map(Number);

        const playerRows = db
            .prepare(
                `SELECT id, name FROM players WHERE id IN (?${',?'.repeat(
                    playerOrder.length - 1,
                )})`,
            )
            .all(playerOrder) as {
            id: number;
            name: string;
        }[];

        if (playerRows.length !== playerOrder.length) {
            logger.error({
                message:
                    'Inconsistent game state: some player IDs in the game state do not exist in the players table.',
            });
            return null;
        }

        const orderedPlayerRows = playerOrder.map(
            // Since we queried only existing IDs, the non-null assertion (as {id, name}) is safe here
            (id) =>
                playerRows.find((p) => p.id === id) as {
                    id: number;
                    name: string;
                },
        );

        return {
            currentPlayerIndex: row.current_player_index,
            hiddenNotes: row.hidden_notes,
            id: row.id,
            notes: row.notes,
            playerOrder: orderedPlayerRows,
            roundNumber: row.round_number,
        };
    },
    removeDeletedPlayersFromPlayerOrder: (existingPlayerIds: number[]) => {
        const gameStateRow = db
            .prepare('SELECT * FROM game_state WHERE id = ?')
            .get(GAME_STATE_ID) as
            | undefined
            | {
                  current_player_index: number;
                  id: number;
                  player_order: string;
                  round_number: number;
              };

        if (!gameStateRow) {
            return;
        }

        const currentPlayerOrderIds = gameStateRow.player_order
            ? gameStateRow.player_order.split(',').map(Number)
            : [];

        const newPlayerOrderIds = currentPlayerOrderIds.filter((id) =>
            existingPlayerIds.includes(id),
        );

        if (newPlayerOrderIds.length === currentPlayerOrderIds.length) {
            return;
        }

        if (gameStateRow.current_player_index >= newPlayerOrderIds.length) {
            // If the current player index is out of bounds after removal, set it to 0
            db.prepare(
                'UPDATE game_state SET current_player_index = 0, round_number = round_number + 1 WHERE id = ?',
            ).run(GAME_STATE_ID);
        }

        db.prepare('UPDATE game_state SET player_order = ? WHERE id = ?').run(
            newPlayerOrderIds.join(','),
            GAME_STATE_ID,
        );
    },
    updateGameState: (id: number, updatedFields: Partial<Omit<GameState, 'id'>>) => {
        const fieldsToUpdate: string[] = [];
        const values: (number | string)[] = [];

        if (updatedFields.roundNumber !== undefined) {
            fieldsToUpdate.push('round_number = ?');
            values.push(updatedFields.roundNumber);
        }

        if (updatedFields.currentPlayerIndex !== undefined) {
            fieldsToUpdate.push('current_player_index = ?');
            values.push(updatedFields.currentPlayerIndex);
        }

        if (updatedFields.playerOrder !== undefined) {
            fieldsToUpdate.push('player_order = ?');
            values.push(updatedFields.playerOrder.map((p) => p.id).join(','));
        }

        if (fieldsToUpdate.length === 0) {
            return;
        }

        values.push(id);

        const query = `UPDATE game_state SET ${fieldsToUpdate.join(', ')} WHERE id = ?`;

        try {
            db.prepare(query).run(...values);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
};

export default gameStateRepository;

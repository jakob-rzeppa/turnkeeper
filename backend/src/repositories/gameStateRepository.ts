import { GameState } from '../entities/GameState.js';

import { SqliteDatabase } from '../database/SqliteDatabase.js';

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
    getGameStateById: (id: number): GameState => {},

    /**
     * Creates a new game state with the given fields
     *
     * @param player_order array of player IDs representing the turn order
     * @throws Conflict if a game state with the same ID already exists
     * @throws ValidationError if the player order contains invalid player IDs, not unique IDs or not all players
     * @throws DatabaseError if there was an unexpected error creating the game state
     */
    createGameState: (player_order: number[]) => {},

    /**
     * Updates the player order of the game state
     *
     * @param game_state_id of the game state to update
     * @param player_order array of player IDs representing the new turn order
     * @throws NotFound if the game state does not exist
     * @throws ValidationError if the player order contains invalid player IDs, not unique IDs or not all players
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updatePlayerOrder: (game_state_id: number, player_order: number[]) => {},

    /**
     * Removes a player from the turn order of the game state
     *
     * @param game_state_id the ID of the game state to update
     * @param player_id the ID of the player to remove from the order
     * @throws NotFound if the game state or player in player_order does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    removePlayerFromOrder: (game_state_id: number, player_id: number) => {},

    /**
     * Advances the game state to the next player's turn.
     *
     * Makes sure to loop back to the first player after the last player
     * and increments the round number accordingly.
     *
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    advanceToNextPlayer: (game_state_id: number) => {},

    /**
     * Updates the notes of the game state.
     *
     * @param game_state_id the game state ID
     * @param notes the new public notes
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updateNotes: (game_state_id: number, notes: string) => {},

    /**
     * Updates the hidden notes of the game state.
     *
     * @param game_state_id the game state ID
     * @param hiddenNotes the new hidden notes
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error during the update
     */
    updateHiddenNotes: (game_state_id: number, hiddenNotes: string) => {},

    /**
     * Deletes the game state with the given ID.
     * It also deletes associated player order entries.
     *
     * @param id of the game state to delete
     * @throws NotFound if the game state does not exist
     * @throws DatabaseError if there was an unexpected error deleting the game state
     */
    deleteGameState: (id: number) => {},
};

export default gameStateRepository;

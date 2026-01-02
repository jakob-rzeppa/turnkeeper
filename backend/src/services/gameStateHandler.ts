import { GameState } from '../entities/GameState.js';
import gameStateRepository from '../repositories/gameStateRepository.js';
import logger from './logger.js';

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;
// Later we might want to support multiple game states (e.g., for multiple concurrent games)
// In that case we would need to save the current game state Id in-memory

const gameStateHandler = {
    /**
     * Gets the current game state
     *
     * DEPENDENCIES:
     * - gameStateRepository.getGameStateById
     *
     * @returns the current game state or null if an error occurred
     */
    getGameState: (): GameState | null => {
        try {
            return gameStateRepository.getGameStateById(GAME_STATE_ID);
        } catch (err: unknown) {
            return null;
        }
    },

    /**
     * Initializes the game state with the given player order
     *
     * DEPENDENCIES:
     * - gameStateRepository.createGameState
     *
     * @param playerOrder the order of player IDs for turn order
     */
    initGameState: (playerOrder: number[]): void => {
        try {
            gameStateRepository.createGameState(playerOrder);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to initialize game state: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Deletes the current game state
     *
     * DEPENDENCIES:
     * - gameStateRepository.deleteGameState
     */
    deleteGameState: (): void => {
        try {
            gameStateRepository.deleteGameState(GAME_STATE_ID);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to delete game state: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Advances the turn to the next player in the turn order
     *
     * DEPENDENCIES:
     * - gameStateRepository.advanceToNextPlayer
     */
    advanceTurn: (): void => {
        try {
            gameStateRepository.advanceTurn(GAME_STATE_ID);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to advance turn: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Updates the public notes of the game state
     *
     * DEPENDENCIES:
     * - gameStateRepository.updateNotes
     *
     * @param newNotes the new public notes
     */
    updateNotes: (newNotes: string): void => {
        try {
            gameStateRepository.updateNotes(GAME_STATE_ID, newNotes);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to update public notes: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },
    /**
     * Updates the hidden notes of the game state
     *
     * DEPENDENCIES:
     * - gameStateRepository.updateHiddenNotes
     *
     * @param newHiddenNotes the new hidden notes
     */
    updateHiddenNotes: (newHiddenNotes: string): void => {
        try {
            gameStateRepository.updateHiddenNotes(GAME_STATE_ID, newHiddenNotes);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to update hidden notes: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Updates the player order in the game state
     *
     * DEPENDENCIES:
     * - gameStateRepository.updatePlayerOrder
     *
     * @param newPlayerIdOrder the new order of player IDs
     */
    updatePlayerOrder: (newPlayerIdOrder: number[]): void => {
        try {
            gameStateRepository.updatePlayerOrder(GAME_STATE_ID, newPlayerIdOrder);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to update player order: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Adds a new player to the turn order at the end
     *
     * DEPENDENCIES:
     * - gameStateRepository.addPlayerToOrder
     *
     * @param playerId the ID of the player to add
     */
    addPlayerToTurnOrder: (playerId: number): void => {
        try {
            gameStateRepository.addPlayerToOrder(GAME_STATE_ID, playerId);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to add player ${playerId} to turn order: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },

    /**
     * Removes a player from the turn order
     *
     * If the player is not in the order, does nothing.
     * If the removed player is the current player, advances to the next player.
     *
     * If the removed player is before the current player in the order,
     * adjusts the current player index accordingly.
     *
     * DEPENDENCIES:
     * - gameStateRepository.removePlayerFromOrder
     *
     * @param playerId the ID of the player to remove
     */
    removePlayerFromTurnOrder: (playerId: number): void => {
        try {
            gameStateRepository.removePlayerFromOrder(GAME_STATE_ID, playerId);

            gameStateRepository.revertTurn(GAME_STATE_ID);
        } catch (err: unknown) {
            logger.error({
                message: `Failed to remove player ${playerId} from turn order: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },
};

export default gameStateHandler;

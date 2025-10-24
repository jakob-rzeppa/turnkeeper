import { GameState } from "shared-types";

import gameStateRepository from "../repositories/gameStateRepository";
import playerRepository from "../repositories/playerRepository";
import logger from "./logger";

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;
// Later we might want to support multiple game states (e.g., for multiple concurrent games)
// In that case we would need to save the current game state Id in-memory

const gameStateHandler = {
    addPlayerToTurnOrder: (playerId: number): void => {
        const playerName = playerRepository.getPlayerNameById(playerId);
        if (!playerName) {
            logger.warn({
                message: `Player with ID ${String(playerId)} not found.`,
            });
            return;
        }

        const gameState = gameStateHandler.getGameState();
        if (!gameState) {
            logger.warn({
                message:
                    "No game state found when attempting to add player to turn order.",
            });
            return;
        }

        if (gameState.playerOrder.find((p) => p.id === playerId)) {
            logger.warn({
                message: `Player with ID ${String(
                    playerId
                )} is already in the turn order.`,
            });
            return;
        }

        gameStateRepository.updateGameState(GAME_STATE_ID, {
            playerOrder: [
                ...gameState.playerOrder,
                { id: playerId, name: playerName },
            ],
        });
    },
    deleteGameState: (): void => {
        gameStateRepository.deleteGameState(GAME_STATE_ID);
    },
    getGameState: (): GameState | null => {
        const gameState = gameStateRepository.getGameStateById(GAME_STATE_ID);

        return gameState;
    },
    initGameState: (newPlayerIdOrder: number[]): void => {
        const playerNames = newPlayerIdOrder.map((id) =>
            playerRepository.getPlayerNameById(id)
        );

        if (!playerNames.every((name) => name !== null)) {
            logger.warn({
                message:
                    "Attempted to initialize game state with non-existing player IDs.",
            });
            return;
        }

        const newGameState: Omit<GameState, "id"> = {
            currentPlayerIndex: 0,
            playerOrder: newPlayerIdOrder.map((id, index) => ({
                id,
                name: playerNames[index],
            })),
            roundNumber: 1,
        };

        gameStateRepository.createGameState(newGameState);
    },
    nextTurn: (): void => {
        const gameState = gameStateHandler.getGameState();

        if (!gameState) {
            logger.warn({
                message:
                    "No game state found when attempting to advance to next turn.",
            });
            return;
        }

        let updatedCurrentPlayerIndex = gameState.currentPlayerIndex + 1;
        let newRoundNumber = gameState.roundNumber;

        if (updatedCurrentPlayerIndex >= gameState.playerOrder.length) {
            newRoundNumber += 1;
            updatedCurrentPlayerIndex = 0;
        }

        gameStateRepository.updateGameState(GAME_STATE_ID, {
            currentPlayerIndex: updatedCurrentPlayerIndex,
            playerOrder: gameState.playerOrder,
            roundNumber: newRoundNumber,
        });
    },
    removeDeletedPlayersFromPlayerOrder: (): void => {
        const gameState = gameStateHandler.getGameState();

        if (!gameState) {
            logger.warn({
                message:
                    "No game state found when attempting to remove deleted players from turn order.",
            });
            return;
        }

        const allPlayerIds = playerRepository.getAllPlayers().map((p) => p.id);

        const updatedPlayerOrder = gameState.playerOrder.filter((player) =>
            allPlayerIds.includes(player.id)
        );

        if (updatedPlayerOrder.length === gameState.playerOrder.length) {
            // No changes needed
            return;
        }

        gameStateRepository.updateGameState(GAME_STATE_ID, {
            playerOrder: updatedPlayerOrder,
        });
    },
    updatePlayerOrder: (newPlayerIdOrder: number[]): void => {
        const playerNames = newPlayerIdOrder.map((id) =>
            playerRepository.getPlayerNameById(id)
        );

        if (!playerNames.every((name) => name !== null)) {
            logger.warn({
                message:
                    "Attempted to update player order with non-existing player IDs.",
            });
            return;
        }

        const newPlayerOrder = newPlayerIdOrder.map((id, index) => ({
            id,
            name: playerNames[index],
        }));

        gameStateRepository.updateGameState(GAME_STATE_ID, {
            playerOrder: newPlayerOrder,
        });
    },
};

export default gameStateHandler;

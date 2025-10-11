import GmController from "../connectionControllers/GmController.js";
import playerRepository from "../repositories/playerRepository.js";
import logger from "./logger.js";

const game = {
    isInitialized: false,
    playerOrder: [] as string[], // array of player IDs in turn order
    round: {
        currentPlayerIndex: 0,
        roundNumber: 0,
    },
};

export const gameloop = {
    addPlayerToTurnOrder: (playerId: string) => {
        if (game.playerOrder.includes(playerId)) {
            logger.warn({
                message: `Player ${playerId} is already in the turn order. Skipping addition.`,
            });
            return;
        }
        game.playerOrder.push(playerId);
        logger.info({
            details: { playerId },
            message: "Player added to turn order.",
        });

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    end: () => {
        if (!game.isInitialized) {
            logger.error({
                message: "Game loop not initialized. Aborting end operation",
            });
            return;
        }

        game.round.roundNumber = 0;
        game.round.currentPlayerIndex = 0;
        game.isInitialized = false;
        game.playerOrder = [];

        logger.info({
            message: "Game ended",
        });

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    getPlayerOrder: () => {
        return game.playerOrder;
    },
    getRoundInformation: () => {
        return {
            currentPlayerIndex: game.round.currentPlayerIndex,
            roundNumber: game.round.roundNumber,
        };
    },
    init: (newPlayerOrder: string[]) => {
        if (game.isInitialized) {
            logger.error({
                message:
                    "Game loop already initialized. Aborting initialization",
            });
            return;
        }

        if (newPlayerOrder.length === 0) {
            logger.error({
                message:
                    "No players to start the game loop provided. Aborting initialization",
            });
            return;
        }

        game.round.roundNumber = 0;
        game.round.currentPlayerIndex = 0;
        game.isInitialized = true;

        // Reset and set player order
        game.playerOrder.splice(0, game.playerOrder.length, ...newPlayerOrder);

        logger.info({
            message: "Game loop initialized",
        });

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    isInitialized: () => {
        return game.isInitialized;
    },
    nextTurn: () => {
        if (game.playerOrder.length === 0) return;

        logger.info({
            details: {
                playerId: game.playerOrder[game.round.currentPlayerIndex],
                roundNumber: game.round.roundNumber,
            },
            message: "End of turn",
        });

        game.round.currentPlayerIndex = game.round.currentPlayerIndex + 1;

        if (game.round.currentPlayerIndex > game.playerOrder.length - 1) {
            game.round.roundNumber += 1;
            game.round.currentPlayerIndex = 0;
            logger.info({
                details: { roundNumber: game.round.roundNumber },
                message: "New round started",
            });
        }

        logger.info({
            details: {
                playerId: game.playerOrder[game.round.currentPlayerIndex],
                roundNumber: game.round.roundNumber,
            },
            message: "Start of turn",
        });

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    // Needs to be called whenever players are deleted to ensure the turn order is accurate
    removeDeletePlayersFromPlayerOrder: () => {
        const allPlayerIds = playerRepository.getAllPlayers().map((p) => p.id);
        game.playerOrder = game.playerOrder.filter((playerId) =>
            allPlayerIds.includes(playerId)
        );

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    setPlayerOrder: (newPlayerOrder: string[]) => {
        if (newPlayerOrder.length === 0) {
            logger.error({
                message:
                    "No players provided to set the player order. Aborting operation.",
            });
            return;
        }

        if (newPlayerOrder.length !== game.playerOrder.length) {
            logger.error({
                message:
                    "The new player order length does not match the current player order length. Aborting operation.",
            });
            return;
        }

        game.playerOrder.splice(0, game.playerOrder.length, ...newPlayerOrder);

        logger.info({
            message: "Player order updated.",
        });

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
};

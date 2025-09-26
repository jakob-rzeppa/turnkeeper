import logger from "./logger.js";

const game = {
    isInitialized: false,
    playerOrder: [] as string[], // array of player IDs in turn order
    round: {
        roundNumber: 0,
        currentPlayerIndex: 0,
    },
};

export const gameloop = {
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
    },
    nextTurn: () => {
        if (game.playerOrder.length === 0) return;

        logger.info({
            message: "End of turn",
            details: {
                roundNumber: game.round.roundNumber,
                playerId: game.playerOrder[game.round.currentPlayerIndex],
            },
        });

        game.round.currentPlayerIndex = game.round.currentPlayerIndex + 1;

        if (game.round.currentPlayerIndex > game.playerOrder.length - 1) {
            game.round.roundNumber += 1;
            game.round.currentPlayerIndex = 0;
            logger.info({
                message: "New round started",
                details: { roundNumber: game.round.roundNumber },
            });
        }

        logger.info({
            message: "Start of turn",
            details: {
                roundNumber: game.round.roundNumber,
                playerId: game.playerOrder[game.round.currentPlayerIndex],
            },
        });
    },
    getRoundInformation: () => {
        return {
            roundNumber: game.round.roundNumber,
            currentPlayerIndex: game.round.currentPlayerIndex,
        };
    },
    getPlayerOrder: () => {
        return game.playerOrder;
    },
};

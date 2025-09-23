import logger from "./logger.js";

// playerIds in order
const playerOrder = [] as string[];

const round = {
    roundNumber: 0,
    currentPlayerIndex: 0,
};

export const gameloop = {
    init: (newPlayerOrder: string[]) => {
        if (newPlayerOrder.length === 0) {
            logger.error({
                message:
                    "No players to start the game loop provided. Aborting initialization",
            });
            return;
        }

        round.roundNumber = 0;
        round.currentPlayerIndex = 0;

        // Reset and set player order
        playerOrder.splice(0, playerOrder.length, ...newPlayerOrder);

        logger.info({
            message: "Game loop initialized",
        });
    },
    nextTurn: () => {
        if (playerOrder.length === 0) return;

        logger.info({
            message: "End of turn",
            details: {
                roundNumber: round.roundNumber,
                playerId: playerOrder[round.currentPlayerIndex],
            },
        });

        round.currentPlayerIndex = round.currentPlayerIndex + 1;

        if (round.currentPlayerIndex > playerOrder.length - 1) {
            round.roundNumber += 1;
            round.currentPlayerIndex = 0;
            logger.info({
                message: "New round started",
                details: { roundNumber: round.roundNumber },
            });
        }

        logger.info({
            message: "Start of turn",
            details: {
                roundNumber: round.roundNumber,
                playerId: playerOrder[round.currentPlayerIndex],
            },
        });
    },
    getRoundInformation: () => {
        return {
            roundNumber: round.roundNumber,
            currentPlayerIndex: round.currentPlayerIndex,
        };
    },
    getPlayerOrder: () => {
        return playerOrder;
    },
};

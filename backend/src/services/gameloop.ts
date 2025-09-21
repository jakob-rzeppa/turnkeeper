// playerIds in order
const playerOrder = [] as string[];

const round = {
    roundNumber: 0,
    currentPlayerIndex: 0,
};

export const gameloop = {
    init: (newPlayerOrder: string[]) => {
        if (newPlayerOrder.length === 0) {
            console.log("No players to start the game loop provided");
            return;
        }

        round.roundNumber = 0;
        round.currentPlayerIndex = 0;

        // Reset and set player order
        playerOrder.splice(0, playerOrder.length, ...newPlayerOrder);

        console.log("Game loop initialized");
    },
    nextTurn: () => {
        if (playerOrder.length === 0) return;

        round.currentPlayerIndex = round.currentPlayerIndex + 1;
        console.log("Next turn");

        if (round.currentPlayerIndex > playerOrder.length - 1) {
            round.roundNumber += 1;
            round.currentPlayerIndex = 0;
            console.log(`Starting round ${round.roundNumber}`);
        }
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

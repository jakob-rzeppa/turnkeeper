import { players } from "./playerService.js";

export default {
    getConnectionId: (name: string) => {
        const player = players.find((player) => player.name === name);
        return player ? player.currentConnectionId : null;
    },
    checkIfPlayerAlreadyConnected: (name: string) => {
        const player = players.find((player) => player.name === name);
        return player ? player.currentConnectionId !== null : false;
    },
    setConnection: (name: string, connectionId: string) => {
        const player = players.find((player) => player.name === name);

        if (!player) {
            throw new Error(`Player ${name} does not exist`);
        }

        if (player.currentConnectionId !== null) {
            throw new Error(`Player ${name} is already connected`);
        }

        player.currentConnectionId = connectionId;
    },
    removeConnection: (name: string) => {
        const player = players.find((player) => player.name === name);
        if (player) {
            player.currentConnectionId = null;
        }
    },
};

import { Player } from "../types/player.js";

export const players: Player[] = [];

export default {
    addPlayer: (name: string) => {
        players.push({ name, currentConnectionId: null, stats: new Map() });
    },
    getPlayer: (name: string) => {
        const player = players.find((player) => player.name === name);
        if (player) {
            return {
                ...player,
                stats: new Map(
                    Array.from(player.stats.entries()).map(([k, v]) => [
                        k,
                        Array.isArray(v) ? [...v] : v,
                    ])
                ),
            };
        }
        return null;
    },
    getPlayers: () => {
        return players.map((player) => ({
            ...player,
            stats: new Map(
                Array.from(player.stats.entries()).map(([k, v]) => [
                    k,
                    Array.isArray(v) ? [...v] : v,
                ])
            ),
        }));
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
    removePlayer: (name: string) => {
        const index = players.findIndex((player) => player.name === name);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

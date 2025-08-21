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
    removePlayer: (name: string) => {
        const index = players.findIndex((player) => player.name === name);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

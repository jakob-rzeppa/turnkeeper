import { Player } from "../types/player.js";

export const players: Player[] = [];

export default {
    addPlayer: (name: string) => {
        players.push({ name, isConnected: false, stats: new Map() });
    },
    checkIfPlayerExists: (name: string) => {
        return players.some((player) => player.name === name);
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
    checkPlayerConnection: (name: string) => {
        const player = players.find((player) => player.name === name);
        return player ? player.isConnected : false;
    },
    setConnection: (name: string, isConnected: boolean) => {
        const player = players.find((player) => player.name === name);
        if (player) {
            player.isConnected = isConnected;
        }
    },
    removePlayer: (name: string) => {
        const index = players.findIndex((player) => player.name === name);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

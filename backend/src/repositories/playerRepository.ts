import { Player, Stat } from "../types/playerTypes.js";
import makePlayerSecret from "../util/makePlayerSecret.js";

export const players: Player[] = [];

const playerRepository = {
    createPlayer: (playerName: string) => {
        // Ensure unique name
        if (players.some((p) => p.name === playerName)) {
            return;
        }

        // Ensure unique ID
        let randomUUID = crypto.randomUUID();
        while (players.some((p) => p.id === randomUUID)) {
            randomUUID = crypto.randomUUID();
        }

        players.push({
            id: randomUUID,
            name: playerName,
            secret: makePlayerSecret({ length: 8 }),
            stats: [],
        });
    },
    createStatForAllPlayers: (stat: Stat) => {
        players.forEach((player) => {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }
            player.stats.push(stat);
        });
    },
    createStatForPlayer: (playerId: string, stat: Stat) => {
        const player = players.find((p) => p.id === playerId);
        if (player) {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }
            player.stats.push(stat);
        }
    },
    deletePlayer: (id: string) => {
        const index = players.findIndex((p) => p.id === id);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
    getAllPlayers: () => players,
    getPlayerById: (id: string) => players.find((p) => p.id === id) || null,
    getPlayerIdByName: (name: string) =>
        players.find((p) => p.name === name)?.id || null,
    getPlayerNameById: (id: string) =>
        players.find((p) => p.id === id)?.name || null,
    removeStatFromPlayer: (playerId: string, statName: string) => {
        const player = players.find((p) => p.id === playerId);
        if (player) {
            player.stats = player.stats.filter((s) => s.name !== statName);
        }
    },
    updatePlayer: (id: string, updatedFields: Partial<Omit<Player, "id">>) => {
        const player = players.find((p) => p.id === id);
        if (player) {
            Object.assign(player, updatedFields);
        }
    },
};

export default playerRepository;

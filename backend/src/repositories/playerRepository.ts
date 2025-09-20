import makePlayerSecret from "../util/makePlayerSecret.js";

export type Stat = {
    name: string;
    value: boolean | number | string | string[];
};

export type Player = {
    id: string;
    name: string;
    secret: string;
    stats: Stat[];
};

export const players: Player[] = [];

const playerRepository = {
    getAllPlayers: () => players,
    getPlayerById: (id: string) => players.find((p) => p.id === id) || null,
    getPlayerIdByName: (name: string) =>
        players.find((p) => p.name === name)?.id || null,
    getPlayerNameById: (id: string) =>
        players.find((p) => p.id === id)?.name || null,
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
    updatePlayer: (id: string, updatedFields: Partial<Omit<Player, "id">>) => {
        const player = players.find((p) => p.id === id);
        if (player) {
            Object.assign(player, updatedFields);
        }
    },
    deletePlayer: (id: string) => {
        const index = players.findIndex((p) => p.id === id);
        if (index !== -1) {
            players.splice(index, 1);
        }
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
    createStatForAllPlayers: (stat: Stat) => {
        players.forEach((player) => {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }
            player.stats.push(stat);
        });
    },
    removeStatFromPlayer: (playerId: string, statName: string) => {
        const player = players.find((p) => p.id === playerId);
        if (player) {
            player.stats = player.stats.filter((s) => s.name !== statName);
        }
    },
};

export default playerRepository;

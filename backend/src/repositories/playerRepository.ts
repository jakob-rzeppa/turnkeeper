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
    removePlayer: (id: string) => {
        const index = players.findIndex((p) => p.id === id);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

export default playerRepository;

import makePlayerSecret from "../util/makePlayerSecret.js";

export type Stat = {
    name: string;
    value: boolean | number | string | string[];
};

export type Player = {
    name: string;
    secret: string;
    stats: Stat[];
};

export const players: Player[] = [];

const playerRepository = {
    getAllPlayers: () => players,
    getPlayerByName: (name: string) =>
        players.find((p) => p.name === name) || null,
    createPlayer: (playerName: string) => {
        if (players.some((p) => p.name === playerName)) {
            return;
        }

        players.push({
            name: playerName,
            secret: makePlayerSecret({ length: 8 }),
            stats: [],
        });
    },
    updatePlayer: (name: string, updatedFields: Partial<Player>) => {
        const player = players.find((p) => p.name === name);
        if (player) {
            Object.assign(player, updatedFields);
        }
    },
    removePlayer: (name: string) => {
        const index = players.findIndex((p) => p.name === name);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

export default playerRepository;

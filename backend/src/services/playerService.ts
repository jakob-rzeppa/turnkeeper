type Stat = {
    name: string;
    value: boolean | number | string | string[];
};

type Player = {
    name: string;
    stats: Stat[];
};

const players: Player[] = [];

const playerService = {
    getAllPlayers: () => players,
    getPlayerByName: (name: string) => players.find((p) => p.name === name),
    addPlayer: (player: Player) => {
        players.push(player);
    },
    updatePlayer: (name: string, stats: Stat[]) => {
        const player = players.find((p) => p.name === name);
        if (player) {
            player.stats = stats;
        }
    },
    removePlayer: (name: string) => {
        const index = players.findIndex((p) => p.name === name);
        if (index !== -1) {
            players.splice(index, 1);
        }
    },
};

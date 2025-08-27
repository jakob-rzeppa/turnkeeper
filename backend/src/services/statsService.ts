import playerService, { Stat } from "./playerService.js";

const statsService = {
    addStatToAllPlayers: (stat: Stat) => {
        playerService.getAllPlayers().forEach((player) => {
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            playerService.updatePlayer(player.name, {
                stats: [...player.stats, stat],
            });
        });
    },
    addStatToPlayer: (playerName: string, stat: Stat) => {
        const player = playerService.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        if (player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        if (player) {
            playerService.updatePlayer(playerName, {
                stats: [...player.stats, stat],
            });
        }
    },
    updateStatOfAllPlayers: (stat: Stat) => {
        playerService.getAllPlayers().forEach((player) => {
            if (!player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            playerService.updatePlayer(player.name, {
                stats: player.stats.map((s) =>
                    s.name === stat.name ? { ...s, value: stat.value } : s
                ),
            });
        });
    },
    updateStatOfPlayer: (playerName: string, stat: Stat) => {
        const player = playerService.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        if (!player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        playerService.updatePlayer(playerName, {
            stats: player.stats.map((s) =>
                s.name === stat.name ? { ...s, value: stat.value } : s
            ),
        });
    },
    removeStatFromPlayer: (playerName: string, statName: string) => {
        const player = playerService.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        playerService.updatePlayer(playerName, {
            stats: player.stats.filter((s) => s.name !== statName),
        });
    },
};

export default statsService;

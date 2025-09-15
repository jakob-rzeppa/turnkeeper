import playerRepository, { Stat } from "../repositories/playerRepository.js";

const statsService = {
    addStatToAllPlayers: (stat: Stat) => {
        playerRepository.getAllPlayers().forEach((player) => {
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            playerRepository.updatePlayer(player.name, {
                stats: [...player.stats, stat],
            });
        });
    },
    addStatToPlayer: (playerName: string, stat: Stat) => {
        const player = playerRepository.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        if (player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        if (player) {
            playerRepository.updatePlayer(playerName, {
                stats: [...player.stats, stat],
            });
        }
    },
    updateStatOfAllPlayers: (stat: Stat) => {
        playerRepository.getAllPlayers().forEach((player) => {
            if (!player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            playerRepository.updatePlayer(player.name, {
                stats: player.stats.map((s) =>
                    s.name === stat.name ? { ...s, value: stat.value } : s
                ),
            });
        });
    },
    updateStatOfPlayer: (playerName: string, stat: Stat) => {
        const player = playerRepository.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        if (!player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        playerRepository.updatePlayer(playerName, {
            stats: player.stats.map((s) =>
                s.name === stat.name ? { ...s, value: stat.value } : s
            ),
        });
    },
    removeStatFromPlayer: (playerName: string, statName: string) => {
        const player = playerRepository.getPlayerByName(playerName);

        if (!player) {
            return;
        }

        playerRepository.updatePlayer(playerName, {
            stats: player.stats.filter((s) => s.name !== statName),
        });
    },
};

export default statsService;

import playerRepository, { Stat } from "../repositories/playerRepository.js";

const statsService = {
    addStatToAllPlayers: (stat: Stat) => {
        playerRepository.getAllPlayers().forEach((player) => {
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            playerRepository.updatePlayer(player.id, {
                stats: [...player.stats, stat],
            });
        });
    },
    addStatToPlayer: (playerId: string, stat: Stat) => {
        const player = playerRepository.getPlayerById(playerId);

        if (!player) {
            return;
        }

        if (player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        if (player) {
            playerRepository.updatePlayer(playerId, {
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
    updateStatOfPlayer: (playerId: string, stat: Stat) => {
        const player = playerRepository.getPlayerById(playerId);

        if (!player) {
            return;
        }

        if (!player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        playerRepository.updatePlayer(playerId, {
            stats: player.stats.map((s) =>
                s.name === stat.name ? { ...s, value: stat.value } : s
            ),
        });
    },
    removeStatFromPlayer: (playerId: string, statName: string) => {
        const player = playerRepository.getPlayerById(playerId);

        if (!player) {
            return;
        }

        playerRepository.updatePlayer(playerId, {
            stats: player.stats.filter((s) => s.name !== statName),
        });
    },
};

export default statsService;

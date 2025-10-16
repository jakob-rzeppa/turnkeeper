// createStatForAllPlayers: (stat: PlayerStat): void => {
//     players.forEach((player) => {
//         // Ensure unique stat name
//         if (player.stats.some((s) => s.name === stat.name)) {
//             return;
//         }
//         player.stats.push(stat);
//     });
// },
// createStatForPlayer: (playerId: string, stat: PlayerStat): void => {
//     const player = players.find((p) => p.id === playerId);
//     if (player) {
//         // Ensure unique stat name
//         if (player.stats.some((s) => s.name === stat.name)) {
//             return;
//         }
//         player.stats.push(stat);
//     }
// },
// removeStatFromPlayer: (playerId: string, statName: string): void => {
//     const player = players.find((p) => p.id === playerId);
//     if (player) {
//         player.stats = player.stats.filter((s) => s.name !== statName);
//     }
// },

import { PlayerStat } from "shared-types";

export const statsRepository = {
    createStatForAllPlayers: (stat: Omit<PlayerStat, "id">): void => {},
    createStatForPlayer: (
        playerId: number,
        stat: Omit<PlayerStat, "id">
    ): void => {},
    removeStatFromPlayer: (playerId: number, statId: number): void => {},
};

import { PlayerStat } from "shared-types";
import { SqliteDatabase } from "../database/SqliteDatabase";
import playerRepository from "./playerRepository";
import logger from "../services/logger";

const db = SqliteDatabase.getInstance();

export const statsRepository = {
    createStatForAllPlayers: (stat: Omit<PlayerStat, "id">): void => {
        const players = playerRepository.getAllPlayers();

        players.forEach((player) => {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            db.prepare(
                "INSERT INTO player_stats (player_id, name, value) VALUES (?, ?, ?)"
            ).run(player.id, stat.name, stat.value);
        });
    },
    createStatForPlayer: (
        playerId: number,
        stat: Omit<PlayerStat, "id">
    ): void => {
        const player = playerRepository.getPlayerById(playerId);

        if (!player) {
            logger.error({
                message: `Player with id ${playerId} not found`,
            });
            return;
        }

        // Ensure unique stat name
        if (player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        db.prepare(
            "INSERT INTO player_stats (player_id, name, value) VALUES (?, ?, ?)"
        ).run(playerId, stat.name, stat.value);
    },
    updateStat: (
        statId: number,
        updatedFields: Partial<Omit<PlayerStat, "id" | "playerId">>
    ): void => {
        db.prepare(
            "UPDATE player_stats SET name = ?, value = ? WHERE id = ?"
        ).run(updatedFields.name, updatedFields.value, statId);
    },
    removeStatFromPlayer: (playerId: number, statId: number): void => {
        db.prepare(
            "DELETE FROM player_stats WHERE id = ? AND player_id = ?"
        ).run(statId, playerId);
    },
};

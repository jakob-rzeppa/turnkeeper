import { PlayerStat } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import logger from '../services/logger.js';
import playerRepository from './playerRepository.js';

const db = SqliteDatabase.getInstance();

export const statsRepository = {
    createStatForAllPlayers: (stat: Omit<PlayerStat, 'id'>): void => {
        const players = playerRepository.getAllPlayers();

        players.forEach((player) => {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === stat.name)) {
                return;
            }

            let value: string;
            switch (typeof stat.value) {
                case 'boolean':
                    value = stat.value ? 'true' : 'false';
                    break;
                case 'number':
                    value = stat.value.toString();
                    break;
                case 'string':
                    value = stat.value;
                    break;
            }

            db.prepare(
                'INSERT INTO player_stats (player_id, name, type, value) VALUES (?, ?, ?, ?)',
            ).run(player.id, stat.name, typeof stat.value, value);
        });
    },
    createStatForPlayer: (playerId: number, stat: Omit<PlayerStat, 'id'>): void => {
        const player = playerRepository.getPlayerById(playerId);

        if (!player) {
            logger.error({
                message: `Player with id ${String(playerId)} not found`,
            });
            return;
        }

        // Ensure unique stat name
        if (player.stats.some((s) => s.name === stat.name)) {
            return;
        }

        let value: string;
        switch (typeof stat.value) {
            case 'boolean':
                value = stat.value ? 'true' : 'false';
                break;
            case 'number':
                value = stat.value.toString();
                break;
            case 'string':
                value = stat.value;
                break;
        }

        db.prepare(
            'INSERT INTO player_stats (player_id, name, type, value) VALUES (?, ?, ?, ?)',
        ).run(playerId, stat.name, typeof stat.value, value);
    },
    removeAllStatsFromPlayer: (playerId: number): void => {
        db.prepare('DELETE FROM player_stats WHERE player_id = ?').run(playerId);
    },
    removeStatFromPlayer: (playerId: number, statId: number): void => {
        db.prepare('DELETE FROM player_stats WHERE id = ? AND player_id = ?').run(statId, playerId);
    },
    updateStatForPlayer: (
        playerId: number,
        statId: number,
        updatedFields: Partial<Omit<PlayerStat, 'id' | 'playerId'>>,
    ): void => {
        const fieldsToUpdate: string[] = [];
        const values: string[] = [];

        // Build the SET clause based on provided fields
        if (updatedFields.name !== undefined && updatedFields.name !== '') {
            fieldsToUpdate.push('name = ?');
            values.push(updatedFields.name);
        }
        if (updatedFields.value !== undefined) {
            fieldsToUpdate.push('type = ?');
            values.push(typeof updatedFields.value);

            fieldsToUpdate.push('value = ?');
            switch (typeof updatedFields.value) {
                case 'boolean':
                    values.push(updatedFields.value ? 'true' : 'false');
                    break;
                case 'number':
                    values.push(updatedFields.value.toString());
                    break;
                case 'string':
                    values.push(updatedFields.value);
                    break;
            }
        }

        if (fieldsToUpdate.length === 0) {
            return;
        }

        // Add the statId and playerId as the last parameter for the WHERE clause
        values.push(statId.toString());
        values.push(playerId.toString());

        db.prepare(
            'UPDATE player_stats SET ' +
                fieldsToUpdate.join(', ') +
                ' WHERE id = ? AND player_id = ?',
        ).run(values);
    },
};

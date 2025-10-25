import type { Player } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import makePlayerSecret from '../util/makePlayerSecret.js';

const db = SqliteDatabase.getInstance();

const playerRepository = {
    createPlayer: (playerName: string): void => {
        const secret = makePlayerSecret({ length: 4 });
        try {
            db.prepare('INSERT INTO players (name, secret) VALUES (?, ?)').run(playerName, secret);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
    deletePlayer: (id: number): void => {
        db.prepare('DELETE FROM players WHERE id = ?').run(id);
    },
    getAllPlayers: (): Player[] => {
        /**
         * Get all players from the database, including their stats.
         * For each stat of a player there is a row in the result set (duplicate player id).
         * The rows are ordered by player id. Therefore we only need to check the next rows if there are more stats for the same player.
         */
        const dbRes = db
            .prepare(
                'SELECT p.id, p.name, p.secret, p.notes, s.id AS statId, s.name AS statName, s.type AS statType, s.value AS statValue FROM players p LEFT JOIN player_stats s ON p.id = s.player_id ORDER BY p.id',
            )
            .all() as {
            id: number;
            name: string;
            notes: string;
            secret: string;
            statId?: number;
            statName?: string;
            statType?: 'boolean' | 'number' | 'string';
            statValue?: string;
        }[];

        const players: Player[] = [];

        for (const row of dbRes) {
            // Create the player if not seen before
            if (players[players.length - 1]?.id !== row.id) {
                players.push({
                    id: row.id,
                    name: row.name,
                    notes: row.notes,
                    secret: row.secret,
                    stats: [],
                });
            }

            // Add the stat if it exists
            if (row.statId && row.statName && row.statType && typeof row.statValue === 'string') {
                let statValue: boolean | number | string;

                switch (row.statType) {
                    case 'boolean':
                        statValue = row.statValue === 'true';
                        break;
                    case 'number':
                        statValue = Number(row.statValue);
                        break;
                    case 'string':
                    default:
                        statValue = row.statValue;
                        break;
                }

                players[players.length - 1].stats.push({
                    id: row.statId,
                    name: row.statName,
                    value: statValue,
                });
            }
        }

        return players;
    },
    getPlayerById: (id: number): null | Player => {
        /**
         * Get a player by id from the database, including the stats.
         * For each stat of the player there is a row in the result set (duplicate player id).
         */
        const dbRes = db
            .prepare(
                'SELECT p.id, p.name, p.secret, s.id AS statId, p.notes, s.name AS statName, s.type AS statType, s.value AS statValue FROM players p LEFT JOIN player_stats s ON p.id = s.player_id WHERE p.id = ?',
            )
            .all(id) as {
            id: number;
            name: string;
            notes: string;
            secret: string;
            statId?: number;
            statName?: string;
            statType?: 'boolean' | 'number' | 'string';
            statValue?: string;
        }[];

        if (dbRes.length === 0) {
            return null;
        }

        const player: Player = {
            id: dbRes[0].id,
            name: dbRes[0].name,
            notes: dbRes[0].notes,
            secret: dbRes[0].secret,
            stats: [],
        };

        for (const row of dbRes) {
            // Add the stat if it exists
            if (row.statId && row.statName && row.statType && typeof row.statValue === 'string') {
                let statValue: boolean | number | string;

                switch (row.statType) {
                    case 'boolean':
                        statValue = row.statValue === 'true';
                        break;
                    case 'number':
                        statValue = Number(row.statValue);
                        break;
                    case 'string':
                    default:
                        statValue = row.statValue;
                        break;
                }

                player.stats.push({
                    id: row.statId,
                    name: row.statName,
                    value: statValue,
                });
            }
        }

        return player;
    },
    getPlayerIdByName: (name: string): null | number => {
        const dbRes = db.prepare('SELECT id FROM players WHERE name = ?').get(name) as
            | undefined
            | { id: number };
        return dbRes ? dbRes.id : null;
    },
    getPlayerNameById: (id: number): null | string => {
        const dbRes = db.prepare('SELECT name FROM players WHERE id = ?').get(id) as
            | undefined
            | { name: string };
        return dbRes ? dbRes.name : null;
    },
    // The update player function is not for updating stats. For updating stats see the statsRepository
    updatePlayer: (id: number, updatedFields: Partial<Omit<Player, 'id' | 'stats'>>): void => {
        const fieldsToUpdate: string[] = [];
        const values: (number | string)[] = [];

        // Build the SET clause based on provided fields
        if (updatedFields.name !== undefined) {
            fieldsToUpdate.push('name = ?');
            values.push(updatedFields.name);
        }
        if (updatedFields.secret !== undefined) {
            fieldsToUpdate.push('secret = ?');
            values.push(updatedFields.secret);
        }
        if (updatedFields.notes !== undefined) {
            fieldsToUpdate.push('notes = ?');
            values.push(updatedFields.notes);
        }

        if (fieldsToUpdate.length === 0) {
            return;
        }

        // Add the id as the last parameter for the WHERE clause
        values.push(id);

        const query = `UPDATE players SET ${fieldsToUpdate.join(', ')} WHERE id = ?`;

        try {
            db.prepare(query).run(...values);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
};

export default playerRepository;

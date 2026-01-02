import type { Player } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import makePlayerSecret from '../util/makePlayerSecret.js';
import { Conflict, DatabaseError, NotFound, ValidationError } from './repositoryErrors.js';

const db = SqliteDatabase.getInstance();

const playerRepository = {
    /**
     * Creates a new player with a randomly generated secret
     *
     * @param playerName of the player to create
     * @throws ValidationError if the player name is empty or invalid
     * @throws Conflict if a player with the same name already exists
     * @throws DatabaseError if there was an unexpected error during creation
     */
    createPlayer: (playerName: string): void => {
        if (!playerName || playerName.trim().length === 0) {
            throw new ValidationError('Player name cannot be empty.');
        }

        const secret = makePlayerSecret({ length: 4 });
        const stmt = db.prepare('INSERT INTO players (name, secret) VALUES (?, ?)');

        try {
            stmt.run(playerName, secret);
        } catch (err: unknown) {
            if (!(err instanceof Error)) {
                throw new DatabaseError('Unexpected error creating player.');
            }

            if (err.message.includes('UNIQUE constraint failed')) {
                throw new Conflict(`Player with name "${playerName}" already exists.`);
            }

            throw new DatabaseError('Unexpected error creating player: ' + err.message);
        }
    },

    /**
     * Deletes a player by their ID
     *
     * @param id of the player to delete
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    deletePlayer: (id: number): void => {
        const stmt = db.prepare('DELETE FROM players WHERE id = ?');

        try {
            const res = stmt.run(id);

            if (res.changes === 0) {
                throw new NotFound(`Player with ID ${id} does not exist.`);
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error deleting player.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error deleting player: ' + err.message);
        }
    },

    /**
     * Retrieves all players from the database, including their stats
     *
     * @returns an array of all players with their associated stats
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getAllPlayers: (): Player[] => {
        /**
         * Get all players from the database, including their stats.
         * For each stat of a player there is a row in the result set (duplicate player id).
         * The rows are ordered by player id. Therefore we only need to check the next rows if there are more stats for the same player.
         */
        const stmt = db.prepare(
            'SELECT p.id, p.name, p.secret, p.notes, p.hidden_notes AS hiddenNotes, s.id AS statId, s.name AS statName, s.type AS statType, s.value AS statValue FROM players p LEFT JOIN player_stats s ON p.id = s.player_id ORDER BY p.id',
        );

        try {
            const dbRes = stmt.all() as {
                hiddenNotes: string;
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
                        hiddenNotes: row.hiddenNotes,
                        id: row.id,
                        name: row.name,
                        notes: row.notes,
                        secret: row.secret,
                        stats: [],
                    });
                }

                // Add the stat if it exists
                if (
                    row.statId &&
                    row.statName &&
                    row.statType &&
                    typeof row.statValue === 'string'
                ) {
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
        } catch (err: unknown) {
            throw new DatabaseError('Unexpected error retrieving players.');
        }
    },

    /**
     * Retrieves a specific player by their ID, including their stats
     *
     * @param id of the player to retrieve
     * @returns the player with their stats
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getPlayerById: (id: number): Player => {
        /**
         * Get a player by id from the database, including the stats.
         * For each stat of the player there is a row in the result set (duplicate player id).
         */
        const stmt = db.prepare(
            'SELECT p.id, p.name, p.secret, p.notes, p.hidden_notes AS hiddenNotes, s.id AS statId, s.name AS statName, s.type AS statType, s.value AS statValue FROM players p LEFT JOIN player_stats s ON p.id = s.player_id WHERE p.id = ?',
        );

        try {
            const dbRes = stmt.all(id) as {
                hiddenNotes: string;
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
                throw new NotFound(`Player with ID ${id} does not exist.`);
            }

            const player: Player = {
                hiddenNotes: dbRes[0].hiddenNotes,
                id: dbRes[0].id,
                name: dbRes[0].name,
                notes: dbRes[0].notes,
                secret: dbRes[0].secret,
                stats: [],
            };

            for (const row of dbRes) {
                // Add the stat if it exists
                if (
                    row.statId &&
                    row.statName &&
                    row.statType &&
                    typeof row.statValue === 'string'
                ) {
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
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error retrieving player.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error retrieving player: ' + err.message);
        }
    },

    /**
     * Retrieves the player ID by their name
     *
     * @param name of the player to search for
     * @returns the player ID
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getPlayerIdByName: (name: string): number => {
        const stmt = db.prepare('SELECT id FROM players WHERE name = ?');

        try {
            const dbRes = stmt.get(name) as undefined | { id: number };
            if (!dbRes) {
                throw new NotFound(`Player with name "${name}" does not exist.`);
            }
            return dbRes.id;
        } catch (err: unknown) {
            if (!(err instanceof Error)) {
                throw new DatabaseError('Unexpected error retrieving player ID by name.');
            }

            if (err instanceof NotFound) throw err;

            throw new DatabaseError(
                'Unexpected error retrieving player ID by name: ' + err.message,
            );
        }
    },

    /**
     * Updates a player's information (excluding stats)
     *
     * @param id of the player to update
     * @param updatedFields partial player object containing fields to update
     * @throws NotFound if the player does not exist
     * @throws Conflict if updating the name would create a duplicate
     * @throws DatabaseError if there was an unexpected error during update
     * @remarks For updating stats, see the statsRepository
     */
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
        if (updatedFields.hiddenNotes !== undefined) {
            fieldsToUpdate.push('hidden_notes = ?');
            values.push(updatedFields.hiddenNotes);
        }

        // Add the id as the last parameter for the WHERE clause
        values.push(id);

        const query = `UPDATE players SET ${fieldsToUpdate.join(', ')} WHERE id = ?`;
        const stmt = db.prepare(query);

        try {
            const result = stmt.run(...values);

            if (result.changes === 0) {
                throw new NotFound(`Player with ID ${id} does not exist.`);
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error updating player.');

            if (err instanceof NotFound) throw err;

            if (err.message.includes('UNIQUE constraint failed'))
                throw new Conflict(`Player with name "${updatedFields.name}" already exists.`);

            throw new DatabaseError('Unexpected error updating player: ' + err.message);
        }
    },
};

export default playerRepository;

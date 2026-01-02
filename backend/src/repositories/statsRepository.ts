import { SqliteDatabase } from '../database/SqliteDatabase.js';
import playerRepository from './playerRepository.js';
import { Conflict, DatabaseError, NotFound, ValidationError } from './repositoryErrors.js';

const db = SqliteDatabase.getInstance();

export const statsRepository = {
    /**
     * Creates a new stat for all players in the database
     *
     * @param stat the stat to create (without id)
     * @throws ValidationError if the stat name is empty or invalid
     * @throws DatabaseError if there was an unexpected error during creation
     * @remarks Skips players that already have a stat with the same name
     */
    createStatForAllPlayers: (name: string, value: string | number | boolean): void => {
        if (!name || name.trim().length === 0) {
            throw new ValidationError('Stat name cannot be empty.');
        }

        const players = playerRepository.getAllPlayers();
        const stmt = db.prepare(
            'INSERT INTO player_stats (player_id, name, type, value) VALUES (?, ?, ?, ?)',
        );

        players.forEach((player) => {
            // Ensure unique stat name
            if (player.stats.some((s) => s.name === name)) {
                return;
            }

            let stringValue: string;
            const valueType = typeof value;

            switch (valueType) {
                case 'boolean':
                    stringValue = value ? 'true' : 'false';
                    break;
                case 'number':
                    stringValue = value.toString();
                    break;
                case 'string':
                    stringValue = value as string;
                    break;
                default:
                    throw new ValidationError('Invalid stat value type.');
            }

            try {
                stmt.run(player.id, name, valueType, stringValue);
            } catch (err: unknown) {
                if (!(err instanceof Error)) {
                    throw new DatabaseError('Unexpected error creating stat for all players.');
                }

                throw new DatabaseError(
                    `Unexpected error creating stat for player ${player.id}: ` + err.message,
                );
            }
        });
    },
    /**
     * Creates a new stat for a specific player
     *
     * @param playerId of the player to create the stat for
     * @param stat the stat to create (without id)
     * @throws ValidationError if the stat name is empty or invalid
     * @throws NotFound if the player does not exist
     * @throws Conflict if the player already has a stat with the same name
     * @throws DatabaseError if there was an unexpected error during creation
     */
    createStatForPlayer: (
        playerId: number,
        name: string,
        value: string | number | boolean,
    ): void => {
        if (!name || name.trim().length === 0) {
            throw new ValidationError('Stat name cannot be empty.');
        }

        const player = playerRepository.getPlayerById(playerId);

        // Ensure unique stat name
        if (player.stats.some((s) => s.name === name)) {
            throw new Conflict(`Player with ID ${playerId} already has a stat named "${name}".`);
        }

        let stringValue: string;
        const valueType = typeof value;

        switch (valueType) {
            case 'boolean':
                stringValue = value ? 'true' : 'false';
                break;
            case 'number':
                stringValue = value.toString();
                break;
            case 'string':
                stringValue = value as string;
                break;
            default:
                throw new ValidationError('Invalid stat value type.');
        }

        const stmt = db.prepare(
            'INSERT INTO player_stats (player_id, name, type, value) VALUES (?, ?, ?, ?)',
        );

        try {
            stmt.run(player.id, name, valueType, stringValue);
        } catch (err: unknown) {
            if (!(err instanceof Error)) {
                throw new DatabaseError('Unexpected error creating stat for player.');
            }

            throw new DatabaseError('Unexpected error creating stat for player: ' + err.message);
        }
    },
    /**
     * Removes all stats from a specific player
     *
     * @param playerId of the player whose stats should be removed
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    removeAllStatsFromPlayer: (playerId: number): void => {
        // Verify player exists (will throw NotFound if not)
        playerRepository.getPlayerById(playerId);

        const stmt = db.prepare('DELETE FROM player_stats WHERE player_id = ?');

        try {
            stmt.run(playerId);
        } catch (err: unknown) {
            throw new DatabaseError(
                'Unexpected error removing stats from player: ' + (err as Error).message,
            );
        }
    },
    /**
     * Removes a specific stat from a player
     *
     * @param playerId of the player whose stat should be removed
     * @param statId of the stat to remove
     * @throws NotFound if the stat does not exist for the player
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    removeStatFromPlayer: (playerId: number, statId: number): void => {
        const stmt = db.prepare('DELETE FROM player_stats WHERE id = ? AND player_id = ?');

        try {
            const result = stmt.run(statId, playerId);

            if (result.changes === 0) {
                throw new NotFound(
                    `Stat with ID ${statId} does not exist for player with ID ${playerId}.`,
                );
            }
        } catch (err: unknown) {
            if (!(err instanceof Error)) {
                throw new DatabaseError('Unexpected error removing stat from player.');
            }

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error removing stat from player: ' + err.message);
        }
    },
    /**
     * Updates a stat for a specific player
     *
     * @param playerId of the player whose stat should be updated
     * @param statId of the stat to update
     * @param updatedFields partial stat object containing fields to update
     * @throws ValidationError if the stat name is empty when provided
     * @throws NotFound if the stat does not exist for the player
     * @throws Conflict if updating the name would create a duplicate stat name for the player
     * @throws DatabaseError if there was an unexpected error during update
     */
    updateStatForPlayer: (
        playerId: number,
        statId: number,
        name?: string,
        value?: string | number | boolean,
    ): void => {
        const fieldsToUpdate: string[] = [];
        const values: string[] = [];

        // Build the SET clause based on provided fields
        if (name !== undefined) {
            if (name.trim().length === 0) {
                throw new ValidationError('Stat name cannot be empty.');
            }
            fieldsToUpdate.push('name = ?');
            values.push(name);
        }
        if (value !== undefined) {
            fieldsToUpdate.push('type = ?');
            values.push(typeof value);
            fieldsToUpdate.push('value = ?');
            switch (typeof value) {
                case 'boolean':
                    values.push(value ? 'true' : 'false');
                    break;
                case 'number':
                    values.push(value.toString());
                    break;
                case 'string':
                    values.push(value);
                    break;
            }
        }

        if (fieldsToUpdate.length === 0) {
            throw new ValidationError('No fields provided to update.');
        }

        // If updating name, check for conflicts first
        if (name !== undefined) {
            try {
                const player = playerRepository.getPlayerById(playerId);
                const duplicateStat = player.stats.find((s) => s.name === name && s.id !== statId);
                if (duplicateStat) {
                    throw new Conflict(
                        `Player with ID ${playerId} already has a stat named "${name}".`,
                    );
                }
            } catch (err: unknown) {
                // If player doesn't exist, let the update handle it
                if (!(err instanceof NotFound)) {
                    throw err;
                }
            }
        }

        // Add the statId and playerId as the last parameter for the WHERE clause
        values.push(statId.toString());
        values.push(playerId.toString());

        const query =
            'UPDATE player_stats SET ' +
            fieldsToUpdate.join(', ') +
            ' WHERE id = ? AND player_id = ?';
        const stmt = db.prepare(query);

        try {
            const result = stmt.run(values);

            if (result.changes === 0) {
                throw new NotFound(
                    `Stat with ID ${statId} does not exist for player with ID ${playerId}.`,
                );
            }
        } catch (err: unknown) {
            if (!(err instanceof Error)) {
                throw new DatabaseError('Unexpected error updating stat for player.');
            }

            if (err instanceof NotFound || err instanceof ValidationError) throw err;

            if (err.message.includes('UNIQUE constraint failed')) {
                throw new Conflict(
                    `Player with ID ${playerId} already has a stat named "${name}".`,
                );
            }

            throw new DatabaseError('Unexpected error updating stat for player: ' + err.message);
        }
    },
};

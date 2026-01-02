import { SqliteDatabase } from '../database/SqliteDatabase.js';
import { Tradable } from '../entities/Tradable.js';
import { Conflict, DatabaseError, NotFound, ValidationError } from './repositoryErrors.js';

const db = SqliteDatabase.getInstance();

/**
 * Repository for tradables, handles all database operations related to tradables
 *
 * Tradables are items that players can trade among each other. Each tradable has an initial quantity
 * defined in the tradables table, and each player has a quantity of each tradable defined
 * in the player_tradables table.
 *
 * We don't need to get tradables via this repository, since they are fetched together with players in playerRepository.
 */
export const tradableRepository = {
    /**
     * Creates a new tradable with the given name, it doesn't initialize any player_tradables entries
     *
     * @param name of the tradable
     * @throws Conflict if a tradable with the same name already exists
     * @throws ValidationError if the name is empty or invalid
     * @throws DatabaseError if there was an unexpected error creating the tradable
     */
    createTradable: (name: string, initialQuantity: number = 0): Tradable => {
        const stmt = db.prepare(
            'INSERT INTO tradables (name, initial_quantity) VALUES (?, ?) RETURNING id, name, initial_quantity',
        );

        try {
            const row = stmt.get(name, initialQuantity) as {
                id: number;
                initial_quantity: number;
                name: string;
            };

            return {
                id: row.id,
                initialQuantity: row.initial_quantity,
                name: row.name,
            };
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error creating tradable.');

            if (err.message.includes('UNIQUE constraint failed'))
                throw new Conflict(`Tradable with name "${name}" already exists.`);

            if (err.message.includes('CHECK constraint failed'))
                throw new ValidationError('Tradable name cannot be empty.');

            throw new DatabaseError('Unexpected error creating tradable.');
        }
    },

    /**
     * Initializes a tradable for all players with quantity specified in tradables.initial_quantity.
     * If the tradable is already initialized for a player, it skips that player.
     *
     * @param tradableId of the tradable to initialize for all players
     * @throws NotFound if the tradable does not exist
     * @throws DatabaseError if there was an unexpected error during initialization
     */
    initializeTradableForAllPlayers: (tradableId: number): void => {
        const tradable = db
            .prepare('SELECT initial_quantity FROM tradables WHERE id = ?')
            .get(tradableId) as { initial_quantity: number } | undefined;

        if (!tradable) {
            throw new NotFound(`Tradable with ID ${tradableId} does not exist.`);
        }

        const stmt = db.prepare(
            `INSERT INTO player_tradables (player_id, tradable_id, quantity)
             SELECT p.id, ?, ? FROM players p
             WHERE NOT EXISTS (
                 SELECT 1 FROM player_tradables pt
                 WHERE pt.player_id = p.id AND pt.tradable_id = ?
             )`,
        );

        try {
            stmt.run(tradableId, tradable.initial_quantity, tradableId);
        } catch (err: unknown) {
            throw new DatabaseError('Unexpected error initializing tradable for all players.');
        }
    },

    /**
     * Sets the quantity of a tradable for a player to the initial value specified in tradables.initial_quantity.
     * If the tradable is already initialized for the player, it does nothing.
     *
     * @param playerId of the player
     * @param tradableId of the tradable to initialize for the player
     * @throws NotFound if the player or tradable does not exist
     * @throws DatabaseError if there was an unexpected error during initialization
     */
    initializeTradableForPlayer: (playerId: number, tradableId: number): void => {
        const player = db.prepare('SELECT id FROM players WHERE id = ?').get(playerId);

        if (!player) {
            throw new NotFound(`Player with ID ${playerId} does not exist.`);
        }

        const tradable = db
            .prepare('SELECT initial_quantity FROM tradables WHERE id = ?')
            .get(tradableId) as { initial_quantity: number } | undefined;

        if (!tradable) {
            throw new NotFound(`Tradable with ID ${tradableId} does not exist.`);
        }

        const stmt = db.prepare(
            'INSERT INTO player_tradables (player_id, tradable_id, quantity) VALUES (?, ?, ?)',
        );

        try {
            stmt.run(playerId, tradableId, tradable.initial_quantity);
        } catch (err: unknown) {
            // If the entry already exists, we silently ignore the error
            if (err instanceof Error && err.message.includes('UNIQUE constraint failed')) {
                return;
            }

            throw new DatabaseError('Unexpected error initializing tradable for player.');
        }
    },

    /**
     * Deletes a tradable by its ID, also removes all associated player_tradables entries.
     *
     * @param tradableId of the tradable to delete
     * @throws NotFound if the tradable does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    deleteTradable: (tradableId: number): void => {
        const tradable = db.prepare('SELECT id FROM tradables WHERE id = ?').get(tradableId);

        if (!tradable) {
            throw new NotFound(`Tradable with ID ${tradableId} does not exist.`);
        }

        const deletePlayerTradablesStmt = db.prepare(
            'DELETE FROM player_tradables WHERE tradable_id = ?',
        );
        const deleteTradableStmt = db.prepare('DELETE FROM tradables WHERE id = ?');

        try {
            // Use a transaction to ensure atomicity
            const transaction = db.transaction(() => {
                deletePlayerTradablesStmt.run(tradableId);
                deleteTradableStmt.run(tradableId);
            });

            transaction();
        } catch (err: unknown) {
            throw new DatabaseError('Unexpected error deleting tradable.');
        }
    },
};

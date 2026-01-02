import { Tradable } from '../entities/Tradable.js';

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
        return { id: 'dummy-id', name, initialQuantity };
    },

    /**
     * Initializes a tradable for all players with quantity specified in tradables.initial_quantity.
     * If the tradable is already initialized for a player, it skips that player.
     *
     * @param tradableId of the tradable to initialize for all players
     * @throws NotFound if the tradable does not exist
     * @throws DatabaseError if there was an unexpected error during initialization
     */
    initializeTradableForAllPlayers: (tradableId: number): void => {},

    /**
     * Sets the quantity of a tradable for a player to the initial value specified in tradables.initial_quantity.
     * If the tradable is already initialized for the player, it does nothing.
     *
     * @param playerId of the player
     * @param tradableId of the tradable to initialize for the player
     * @throws NotFound if the player or tradable does not exist
     * @throws DatabaseError if there was an unexpected error during initialization
     */
    initializeTradableForPlayer: (playerId: number, tradableId: number): void => {},

    /**
     * Deletes a tradable by its ID, also removes all associated player_tradables entries.
     *
     * @param tradableId of the tradable to delete
     * @throws NotFound if the tradable does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    deleteTradable: (tradableId: number): void => {},
};

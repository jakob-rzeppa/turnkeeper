import { SqliteDatabase } from '../database/SqliteDatabase.js';
import { Message } from '../entities/Message.js';
import { DatabaseError, NotFound, ValidationError } from './repositoryErrors.js';

const db = SqliteDatabase.getInstance();

const messageRepository = {
    /**
     * Retrieves all messages grouped by player ID
     *
     * @returns a record where each key is a player ID and the value is an array of messages for that player
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getAllMessagesGroupedByPlayerId: (): Record<number, Message[]> => {
        const stmt = db.prepare(
            `SELECT id, player_id, content, timestamp, send_by 
            FROM messages 
            ORDER BY timestamp ASC`,
        );

        try {
            const rows = stmt.all() as {
                id: number;
                player_id: number;
                content: string;
                timestamp: string;
                send_by: 'player' | 'gm';
            }[];

            const groupedMessages: Record<number, Message[]> = {};

            for (const row of rows) {
                const message: Message = {
                    id: row.id,
                    playerId: row.player_id,
                    content: row.content,
                    timestamp: new Date(row.timestamp),
                    sendBy: row.send_by,
                };

                if (!groupedMessages[row.player_id]) {
                    groupedMessages[row.player_id] = [];
                }
                groupedMessages[row.player_id].push(message);
            }

            return groupedMessages;
        } catch (err: unknown) {
            throw new DatabaseError('Unexpected error retrieving messages.');
        }
    },

    /**
     * Retrieves all messages for a specific player by their ID
     *
     * @param playerId of the player whose messages should be retrieved
     * @returns an array of messages for the specified player
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during retrieval
     */
    getMessagesByPlayerId: (playerId: number): Message[] => {
        const playerExists = db.prepare('SELECT 1 FROM players WHERE id = ?').get(playerId);

        if (!playerExists) {
            throw new NotFound(`Player with ID ${playerId} does not exist.`);
        }

        const stmt = db.prepare(
            `SELECT id, player_id, content, timestamp, send_by 
            FROM messages 
            WHERE player_id = ? 
            ORDER BY timestamp ASC`,
        );

        try {
            const rows = stmt.all(playerId) as {
                id: number;
                player_id: number;
                content: string;
                timestamp: string;
                send_by: 'player' | 'gm';
            }[];

            return rows.map((row) => ({
                id: row.id,
                playerId: row.player_id,
                content: row.content,
                timestamp: new Date(row.timestamp),
                sendBy: row.send_by,
            }));
        } catch (err: unknown) {
            throw new DatabaseError('Unexpected error retrieving messages for player.');
        }
    },

    /**
     * Creates a new message in the database
     *
     * @param message to create
     * @throws ValidationError if the message data is empty or sendBy is invalid
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error creating the message
     * @returns the created message with ID and timestamp
     */
    createMessage: (message: Omit<Message, 'id' | 'timestamp'>): Message => {
        // Validate sendBy before attempting insert
        if (!['gm', 'player', 'system'].includes(message.sendBy)) {
            throw new ValidationError(`Invalid sendBy value: ${message.sendBy}`);
        }

        const stmt = db.prepare(
            `INSERT INTO messages (player_id, content, send_by) VALUES (?, ?, ?) 
            RETURNING id, player_id, content, timestamp, send_by`,
        );

        try {
            const row = stmt.get(message.playerId, message.content, message.sendBy) as {
                id: number;
                player_id: number;
                content: string;
                timestamp: string;
                send_by: 'player' | 'gm';
            };

            return {
                id: row.id,
                playerId: row.player_id,
                content: row.content,
                timestamp: new Date(row.timestamp),
                sendBy: row.send_by,
            };
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error creating message.');

            if (err.message.includes('FOREIGN KEY constraint failed'))
                throw new NotFound(`Player with ID ${message.playerId} does not exist.`);

            if (err.message.includes('CHECK constraint failed'))
                throw new ValidationError('Message content cannot be empty.');

            throw new DatabaseError('Unexpected error creating message: ' + err.message);
        }
    },

    /**
     * Deletes a message by its ID
     *
     * @param messageId of the message to be deleted
     * @throws NotFound if the message does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    deleteMessageById: (messageId: number): void => {
        const stmt = db.prepare('DELETE FROM messages WHERE id = ?');

        try {
            const result = stmt.run(messageId);

            if (result.changes === 0) {
                throw new NotFound(`Message with ID ${messageId} does not exist.`);
            }
        } catch (err: unknown) {
            if (!(err instanceof Error))
                throw new DatabaseError('Unexpected error deleting message.');

            if (err instanceof NotFound) throw err;

            throw new DatabaseError('Unexpected error deleting message: ' + err.message);
        }
    },

    /**
     * Deletes all messages sent by a specific player
     *
     * @param playerId of the player whose messages should be deleted
     * @throws NotFound if the player does not exist
     * @throws DatabaseError if there was an unexpected error during deletion
     */
    deleteAllMessagesByPlayerId: (playerId: number): void => {
        const playerExists = db.prepare('SELECT 1 FROM players WHERE id = ?').get(playerId);

        if (!playerExists) {
            throw new NotFound(`Player with ID ${playerId} does not exist.`);
        }

        const stmt = db.prepare('DELETE FROM messages WHERE player_id = ?');

        try {
            stmt.run(playerId);
        } catch (err: unknown) {
            throw new DatabaseError(
                'Unexpected error deleting messages for player: ' + (err as Error).message,
            );
        }
    },
};

export default messageRepository;

import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase.js';
import messageRepository from '../../repositories/messageRepository.js';
import { NotFound, ValidationError } from '../../repositories/repositoryErrors.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('Message Repository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('createMessage', () => {
        it('should create a new message and return it with id and timestamp', () => {
            // Create a player first
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const message = messageRepository.createMessage({
                content: 'Hello World',
                playerId: 1,
                sendBy: 'player',
            });

            expect(message).toBeDefined();
            expect(message.id).toBeDefined();
            expect(message.content).toBe('Hello World');
            expect(message.playerId).toBe(1);
            expect(message.sendBy).toBe('player');
            expect(message.timestamp).toBeInstanceOf(Date);

            // Verify it is stored in the database
            const storedMessage = db.prepare('SELECT * FROM messages WHERE id = ?').get(message.id);
            expect(storedMessage).toBeDefined();
            expect(storedMessage).toMatchObject({
                id: message.id,
                content: 'Hello World',
                player_id: 1,
                send_by: 'player',
            });
        });

        it('should create messages with different sendBy values', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const playerMessage = messageRepository.createMessage({
                content: 'From player',
                playerId: 1,
                sendBy: 'player',
            });
            const gmMessage = messageRepository.createMessage({
                content: 'From GM',
                playerId: 1,
                sendBy: 'gm',
            });

            expect(playerMessage.sendBy).toBe('player');
            expect(gmMessage.sendBy).toBe('gm');
        });

        it('should throw ValidationError if sendBy is invalid', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() =>
                messageRepository.createMessage({
                    content: 'Test',
                    playerId: 1,
                    sendBy: 'invalid' as any,
                }),
            ).toThrow(ValidationError);
        });

        it('should throw ValidationError if content is empty', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() =>
                messageRepository.createMessage({
                    content: '',
                    playerId: 1,
                    sendBy: 'player',
                }),
            ).toThrow(ValidationError);
        });

        it('should throw NotFound if player does not exist', () => {
            expect(() =>
                messageRepository.createMessage({
                    content: 'Test',
                    playerId: 999,
                    sendBy: 'player',
                }),
            ).toThrow(NotFound);
        });
    });

    describe('deleteAllMessagesByPlayerId', () => {
        it('should delete all messages for a specific player', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create messages for both players
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'player', 'Message 1')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'gm', 'Message 2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (2, 'player', 'Message 3')",
            );

            messageRepository.deleteAllMessagesByPlayerId(1);

            // Verify Alice's messages are deleted
            const aliceMessages = db.prepare('SELECT * FROM messages WHERE player_id = 1').all();
            expect(aliceMessages).toHaveLength(0);

            // Verify Bob's messages remain
            const bobMessages = db.prepare('SELECT * FROM messages WHERE player_id = 2').all();
            expect(bobMessages).toHaveLength(1);
        });

        it('should not throw error if player has no messages', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            expect(() => messageRepository.deleteAllMessagesByPlayerId(1)).not.toThrow();
        });

        it('should throw NotFound if player does not exist', () => {
            expect(() => messageRepository.deleteAllMessagesByPlayerId(999)).toThrow(NotFound);
        });
    });

    describe('deleteMessageById', () => {
        it('should delete a specific message by its ID', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO messages (id, player_id, send_by, content) VALUES (1, 1, 'player', 'Message 1')",
            );
            db.exec(
                "INSERT INTO messages (id, player_id, send_by, content) VALUES (2, 1, 'player', 'Message 2')",
            );

            messageRepository.deleteMessageById(1);

            // Verify message 1 is deleted
            const message1 = db.prepare('SELECT * FROM messages WHERE id = 1').get();
            expect(message1).toBeUndefined();

            // Verify message 2 remains
            const message2 = db.prepare('SELECT * FROM messages WHERE id = 2').get();
            expect(message2).toBeDefined();
        });

        it('should throw NotFound if message does not exist', () => {
            expect(() => messageRepository.deleteMessageById(999)).toThrow(NotFound);
        });
    });

    describe('getAllMessagesGroupedByPlayerId', () => {
        it('should return messages grouped by player ID', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create messages
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'player', 'Alice Message 1')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'gm', 'Alice Message 2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (2, 'player', 'Bob Message 1')",
            );

            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();

            expect(groupedMessages[1]).toHaveLength(2);
            expect(groupedMessages[1][0].content).toBe('Alice Message 1');
            expect(groupedMessages[1][1].content).toBe('Alice Message 2');
            expect(groupedMessages[2]).toHaveLength(1);
            expect(groupedMessages[2][0].content).toBe('Bob Message 1');
        });

        it('should return empty object if no messages exist', () => {
            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();
            expect(groupedMessages).toEqual({});
        });

        it('should return messages in chronological order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Insert messages with different timestamps
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'First', '2024-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Second', '2024-01-01 11:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Third', '2024-01-01 12:00:00')",
            );

            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();

            expect(groupedMessages[1][0].content).toBe('First');
            expect(groupedMessages[1][1].content).toBe('Second');
            expect(groupedMessages[1][2].content).toBe('Third');
        });
    });

    describe('getMessagesByPlayerId', () => {
        it('should return all messages for a specific player', () => {
            // Create players
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec("INSERT INTO players (id, name, secret) VALUES (2, 'Bob', 'secret2')");

            // Create messages
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'player', 'Alice Message 1')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'gm', 'Alice Message 2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (2, 'player', 'Bob Message')",
            );

            const aliceMessages = messageRepository.getMessagesByPlayerId(1);

            expect(aliceMessages).toHaveLength(2);
            expect(aliceMessages[0].content).toBe('Alice Message 1');
            expect(aliceMessages[0].playerId).toBe(1);
            expect(aliceMessages[1].content).toBe('Alice Message 2');
            expect(aliceMessages[1].playerId).toBe(1);
        });

        it('should return empty array if player has no messages', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const messages = messageRepository.getMessagesByPlayerId(1);
            expect(messages).toEqual([]);
        });

        it('should throw NotFound if player does not exist', () => {
            expect(() => messageRepository.getMessagesByPlayerId(999)).toThrow(NotFound);
        });

        it('should return messages in chronological order', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            // Insert messages with different timestamps
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'First', '2024-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Second', '2024-01-01 11:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Third', '2024-01-01 12:00:00')",
            );

            const messages = messageRepository.getMessagesByPlayerId(1);

            expect(messages[0].content).toBe('First');
            expect(messages[1].content).toBe('Second');
            expect(messages[2].content).toBe('Third');
        });

        it('should properly parse message timestamps', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO messages (player_id, send_by, content) VALUES (1, 'player', 'Test')",
            );

            const messages = messageRepository.getMessagesByPlayerId(1);

            expect(messages[0].timestamp).toBeInstanceOf(Date);
        });
    });
});

import { Message } from 'shared-types';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SqliteDatabase } from '../../database/SqliteDatabase.js';
import messageRepository from '../../repositories/messageRepository.js';
import logger from '../../services/logger.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

vi.mock('../../services/logger.ts', () => ({
    default: {
        error: vi.fn(),
    },
}));

describe('Message Repository', () => {
    const db = SqliteDatabase.getInstance();

    beforeEach(() => {
        db.dropTables();
        db.initializeTables();
    });

    describe('createMessage', () => {
        it('should create a new message in the database', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");

            const newMessage: Omit<Message, 'id' | 'timestamp'> = {
                content: 'Hello, world!',
                playerId: 1,
                sendBy: 'player',
            };

            messageRepository.createMessage(newMessage);

            const messages = db.prepare('SELECT * FROM messages').all();
            expect(messages.length).toBe(1);
            expect(messages[0]).toEqual({
                content: newMessage.content,
                id: 1,
                player_id: newMessage.playerId,
                send_by: newMessage.sendBy,
                timestamp: expect.any(String) as unknown,
            });
        });

        it('should handle messages meant only for the GM', () => {
            const newMessage: Omit<Message, 'id' | 'timestamp'> = {
                content: 'This is a GM-only message.',
                playerId: null,
                sendBy: 'system',
            };

            messageRepository.createMessage(newMessage);

            const messages = db.prepare('SELECT * FROM messages').all();
            expect(messages.length).toBe(1);
            expect(messages[0]).toEqual({
                content: newMessage.content,
                id: 1,
                player_id: null,
                send_by: newMessage.sendBy,
                timestamp: expect.any(String) as unknown,
            });
        });

        it('should log an error for invalid sendBy values', () => {
            const newMessage: Omit<Message, 'id' | 'timestamp'> = {
                content: 'This should fail.',
                playerId: 1,
                // @ts-expect-error Testing invalid value
                sendBy: 'invalid_sender',
            };

            expect(() => {
                messageRepository.createMessage(newMessage);
            }).not.toThrow();
            expect(logger.error).toHaveBeenCalledWith({
                details: {
                    content: 'This should fail.',
                    playerId: 1,
                    sendBy: 'invalid_sender',
                },
                message: 'Invalid sendBy value: invalid_sender',
            });
        });
    });

    describe('getMessagesByPlayerId', () => {
        it('should return an empty array when no messages exist for the player', () => {
            const messages = messageRepository.getMessagesByPlayerId(1);
            expect(messages).toEqual([]);
        });

        it('should return messages for the specified player', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'system', 'System message', '2023-01-01 10:01:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Hello', '2023-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (2, 'player', 'Other player message', '2023-01-01 10:03:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'gm', 'Gm message', '2023-01-01 10:02:00')",
            );

            const messages = messageRepository.getMessagesByPlayerId(1);
            expect(messages.length).toBe(3);
            expect(messages).toContainEqual(
                expect.objectContaining({
                    content: 'Hello',
                    playerId: 1,
                    sendBy: 'player',
                    timestamp: new Date('2023-01-01 10:00:00'),
                }),
            );
            expect(messages).toContainEqual(
                expect.objectContaining({
                    content: 'Gm message',
                    playerId: 1,
                    sendBy: 'gm',
                    timestamp: new Date('2023-01-01 10:02:00'),
                }),
            );
            expect(messages).toContainEqual(
                expect.objectContaining({
                    content: 'System message',
                    playerId: 1,
                    sendBy: 'system',
                    timestamp: new Date('2023-01-01 10:01:00'),
                }),
            );
            expect(
                messages[0].timestamp <= messages[1].timestamp &&
                    messages[1].timestamp <= messages[2].timestamp,
            ).toBe(true);
        });

        it('should handle messages with null playerId (GM-only messages) by not returning them', () => {
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (NULL, 'system', 'GM-only message', '2023-01-01 10:00:00')",
            );

            const messages = messageRepository.getMessagesByPlayerId(1);
            expect(messages).toEqual([]);
        });
    });

    describe('getAllMessagesGroupedByPlayerId', () => {
        it('should return all messages grouped by player ID', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Hello Alice', '2023-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (2, 'player', 'Hello Bob', '2023-01-01 10:05:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'gm', 'GM message for Alice', '2023-01-01 10:10:00')",
            );

            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();
            expect(Object.keys(groupedMessages).length).toBe(2);
            expect(groupedMessages[1].length).toBe(2);
            expect(groupedMessages[2].length).toBe(1);

            expect(groupedMessages[1]).toContainEqual(
                expect.objectContaining({
                    content: 'Hello Alice',
                    playerId: 1,
                    sendBy: 'player',
                    timestamp: new Date('2023-01-01 10:00:00'),
                }),
            );
            expect(groupedMessages[1]).toContainEqual(
                expect.objectContaining({
                    content: 'GM message for Alice',
                    playerId: 1,
                    sendBy: 'gm',
                    timestamp: new Date('2023-01-01 10:10:00'),
                }),
            );
            expect(groupedMessages[2]).toContainEqual(
                expect.objectContaining({
                    content: 'Hello Bob',
                    playerId: 2,
                    sendBy: 'player',
                    timestamp: new Date('2023-01-01 10:05:00'),
                }),
            );

            expect(groupedMessages[1][0].timestamp <= groupedMessages[1][1].timestamp).toBe(true);
        });

        it('should return an empty object when there are no messages', () => {
            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();
            expect(groupedMessages).toEqual({});
        });

        it('should handle messages with null playerId (GM-only messages)', () => {
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (NULL, 'system', 'GM-only message', '2023-01-01 10:00:00')",
            );

            const groupedMessages = messageRepository.getAllMessagesGroupedByPlayerId();
            expect(Object.keys(groupedMessages).length).toBe(0);
        });
    });

    describe('deleteAllMessagesByPlayerId', () => {
        it('should delete all messages for the specified player ID', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Hello Alice', '2023-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (2, 'player', 'Hello Bob', '2023-01-01 10:05:00')",
            );

            messageRepository.deleteAllMessagesByPlayerId(1);

            const remainingMessages = db.prepare('SELECT * FROM messages').all() as {
                content: string;
                id: number;
                player_id: null | number;
                send_by: 'gm' | 'player' | 'system';
                timestamp: string;
            }[];
            expect(remainingMessages.length).toBe(1);
            expect(remainingMessages[0].player_id).toBe(2);
        });

        it('should do nothing if there are no messages for the specified player ID', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Hello Alice', '2023-01-01 10:00:00')",
            );

            messageRepository.deleteAllMessagesByPlayerId(2); // No messages for player ID 2

            const remainingMessages = db.prepare('SELECT * FROM messages').all() as {
                content: string;
                id: number;
                player_id: null | number;
                send_by: 'gm' | 'player' | 'system';
                timestamp: string;
            }[];
            expect(remainingMessages.length).toBe(1);
            expect(remainingMessages[0].player_id).toBe(1);
        });
    });

    describe('deleteMessageById', () => {
        it('should delete the message with the specified ID', () => {
            db.exec(
                "INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1'), (2, 'Bob', 'secret2')",
            );
            db.exec(
                "INSERT INTO messages (id, player_id, send_by, content, timestamp) VALUES (1, 1, 'player', 'Hello Alice', '2023-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (id, player_id, send_by, content, timestamp) VALUES (2, 2, 'player', 'Hello Bob', '2023-01-01 10:05:00')",
            );

            messageRepository.deleteMessageById(1);

            const remainingMessages = db.prepare('SELECT * FROM messages').all() as {
                content: string;
                id: number;
                player_id: null | number;
                send_by: 'gm' | 'player' | 'system';
                timestamp: string;
            }[];
            expect(remainingMessages.length).toBe(1);
            expect(remainingMessages[0].id).toBe(2);
        });

        it('should do nothing if the message ID does not exist', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO messages (id, player_id, send_by, content, timestamp) VALUES (1, 1, 'player', 'Hello Alice', '2023-01-01 10:00:00')",
            );

            messageRepository.deleteMessageById(999); // Non-existent ID

            const remainingMessages = db.prepare('SELECT * FROM messages').all() as {
                content: string;
                id: number;
                player_id: null | number;
                send_by: 'gm' | 'player' | 'system';
                timestamp: string;
            }[];
            expect(remainingMessages.length).toBe(1);
            expect(remainingMessages[0].id).toBe(1);
        });
    });

    describe('getMessagesWithoutPlayerId', () => {
        it('should return an empty array when no GM-only messages exist', () => {
            const messages = messageRepository.getMessagesWithoutPlayerId();
            expect(messages).toEqual([]);
        });

        it('should return GM-only messages', () => {
            db.exec("INSERT INTO players (id, name, secret) VALUES (1, 'Alice', 'secret1')");
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (NULL, 'system', 'GM-only message 1', '2023-01-01 10:00:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (NULL, 'system', 'GM-only message 2', '2023-01-01 10:05:00')",
            );
            db.exec(
                "INSERT INTO messages (player_id, send_by, content, timestamp) VALUES (1, 'player', 'Player message', '2023-01-01 10:10:00')",
            );

            const messages = messageRepository.getMessagesWithoutPlayerId();
            expect(messages.length).toBe(2);
            expect(messages).toContainEqual(
                expect.objectContaining({
                    content: 'GM-only message 1',
                    playerId: null,
                    sendBy: 'system',
                    timestamp: new Date('2023-01-01 10:00:00'),
                }),
            );
            expect(messages).toContainEqual(
                expect.objectContaining({
                    content: 'GM-only message 2',
                    playerId: null,
                    sendBy: 'system',
                    timestamp: new Date('2023-01-01 10:05:00'),
                }),
            );

            expect(messages[0].timestamp <= messages[1].timestamp).toBe(true);
        });
    });
});

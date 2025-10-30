import { Message } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase';
import logger from '../services/logger';

const db = SqliteDatabase.getInstance();

const messageRepository = {
    createMessage: (message: Omit<Message, 'id' | 'timestamp'>): void => {
        if (!['gm', 'player', 'system'].includes(message.sendBy)) {
            logger.error({
                details: {
                    content: message.content,
                    playerId: message.playerId,
                    sendBy: message.sendBy,
                },
                message: `Invalid sendBy value: ${message.sendBy}`,
            });
            return;
        }

        try {
            const stmt = db.prepare(
                `INSERT INTO messages (player_id, send_by, content) 
                 VALUES (?, ?, ?)`,
            );
            stmt.run(message.playerId, message.sendBy, message.content);
        } catch {
            // Handle error silently
        }
    },
    deleteAllMessagesByPlayerId: (playerId: number): void => {
        const stmt = db.prepare(`DELETE FROM messages WHERE player_id = ?`);
        stmt.run(playerId);
    },
    deleteMessageById: (messageId: number): void => {
        const stmt = db.prepare(`DELETE FROM messages WHERE id = ?`);
        stmt.run(messageId);
    },
    getAllMessagesGroupedByPlayerId: (): Record<number, Message[]> => {
        const stmt = db.prepare(`SELECT * FROM messages ORDER BY timestamp ASC`);
        const rows = stmt.all() as {
            content: string;
            id: number;
            player_id: null | number;
            send_by: 'gm' | 'player' | 'system';
            timestamp: string;
        }[];

        const groupedMessages: Record<number, Message[]> = {};
        rows.forEach((row) => {
            const message: Message = {
                content: row.content,
                id: row.id,
                playerId: row.player_id,
                sendBy: row.send_by,
                timestamp: new Date(row.timestamp),
            };

            // Skip messages with null playerId (GM-only messages)
            if (row.player_id === null) {
                return;
            }

            if (!Object.keys(groupedMessages).includes(row.player_id.toString())) {
                groupedMessages[row.player_id] = [];
            }

            groupedMessages[row.player_id].push(message);
        });

        return groupedMessages;
    },
    getMessagesByPlayerId: (playerId: number): Message[] => {
        const stmt = db.prepare(
            `SELECT * FROM messages WHERE player_id = ? ORDER BY timestamp ASC`,
        );
        const rows = stmt.all(playerId) as {
            content: string;
            id: number;
            player_id: null | number;
            send_by: 'gm' | 'player' | 'system';
            timestamp: string;
        }[];

        if (rows.length === 0) {
            return [];
        }

        const messages: Message[] = rows.map((row) => ({
            content: row.content,
            id: row.id,
            playerId: row.player_id,
            sendBy: row.send_by,
            timestamp: new Date(row.timestamp),
        }));

        return messages;
    },
};

export default messageRepository;

import { Message } from 'shared-types';

import { SqliteDatabase } from '../database/SqliteDatabase.js';
import logger from '../services/logger.js';

const db = SqliteDatabase.getInstance();

const messageRepository = {
    createMessage: (message: Omit<Message, 'id' | 'timestamp'>): Message | null => {
        if (!['gm', 'player', 'system'].includes(message.sendBy)) {
            logger.error({
                details: {
                    content: message.content,
                    playerId: message.playerId,
                    sendBy: message.sendBy,
                },
                message: `Invalid sendBy value: ${message.sendBy}`,
            });
            return null;
        }

        try {
            const stmt = db.prepare(
                `INSERT INTO messages (player_id, send_by, content) 
                 VALUES (?, ?, ?)
                 RETURNING id, player_id, send_by, content, timestamp`,
            );
            const row = stmt.get(message.playerId, message.sendBy, message.content) as
                | {
                      content: string;
                      id: number;
                      player_id: number;
                      send_by: 'gm' | 'player' | 'system';
                      timestamp: string;
                  }
                | undefined;

            return row
                ? {
                      content: row.content,
                      id: row.id,
                      playerId: row.player_id,
                      sendBy: row.send_by,
                      timestamp: new Date(row.timestamp),
                  }
                : null;
        } catch {
            // Handle error silently
            return null;
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
            player_id: number;
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
            player_id: number;
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

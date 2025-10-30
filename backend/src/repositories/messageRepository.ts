import { Message } from 'shared-types';
import { SqliteDatabase } from '../database/SqliteDatabase';
import logger from '../services/logger';

const db = SqliteDatabase.getInstance();

const messageRepository = {
    createMessage: (message: Omit<Message, 'id' | 'timestamp'>): void => {
        if (!['player', 'system', 'gm'].includes(message.sendBy)) {
            logger.error({
                message: `Invalid sendBy value: ${message.sendBy}`,
                details: {
                    playerId: message.playerId,
                    content: message.content,
                    sendBy: message.sendBy,
                },
            });
            return;
        }

        const stmt = db.prepare(
            `INSERT INTO messages (player_id, send_by, content) 
             VALUES (?, ?, ?)`,
        );
        stmt.run(message.playerId, message.sendBy, message.content);
    },
    getMessagesByPlayerId: (playerId: number): Message[] => {
        const stmt = db.prepare(
            `SELECT * FROM messages WHERE player_id = ? ORDER BY timestamp ASC`,
        );
        const rows = stmt.all(playerId) as {
            id: number;
            player_id: number | null;
            send_by: 'player' | 'system' | 'gm';
            content: string;
            timestamp: string;
        }[];

        if (rows.length === 0) {
            return [];
        }

        const messages: Message[] = rows.map((row) => ({
            id: row.id,
            playerId: row.player_id,
            sendBy: row.send_by,
            content: row.content,
            timestamp: new Date(row.timestamp),
        }));

        return messages;
    },
    getAllMessagesGroupedByPlayerId: (): { [playerId: number]: Message[] } => {
        const stmt = db.prepare(`SELECT * FROM messages ORDER BY timestamp ASC`);
        const rows = stmt.all() as {
            id: number;
            player_id: number | null;
            send_by: 'player' | 'system' | 'gm';
            content: string;
            timestamp: string;
        }[];

        const groupedMessages: { [playerId: number]: Message[] } = {};
        rows.forEach((row) => {
            const message: Message = {
                id: row.id,
                playerId: row.player_id,
                sendBy: row.send_by,
                content: row.content,
                timestamp: new Date(row.timestamp),
            };

            // Skip messages with null playerId (GM-only messages)
            if (row.player_id === null) {
                return;
            }

            if (!groupedMessages[row.player_id]) {
                groupedMessages[row.player_id] = [];
            }

            groupedMessages[row.player_id].push(message);
        });

        return groupedMessages;
    },
    deleteAllMessagesByPlayerId: (playerId: number): void => {
        const stmt = db.prepare(`DELETE FROM messages WHERE player_id = ?`);
        stmt.run(playerId);
    },
    deleteMessageById: (messageId: number): void => {
        const stmt = db.prepare(`DELETE FROM messages WHERE id = ?`);
        stmt.run(messageId);
    },
};

export default messageRepository;

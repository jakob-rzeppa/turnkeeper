import { Socket } from "socket.io";

import playerRepository from "../repositories/playerRepository";

export const extractUserCredentials = (
    socket: Socket
): null | { playerId: number; playerSecret: string } => {
    const playerName: unknown = socket.handshake.auth.playerName;
    const playerSecret: unknown = socket.handshake.auth.playerSecret;

    if (!playerName || !playerSecret) return null;
    if (typeof playerName !== "string" || typeof playerSecret !== "string")
        return null;

    const playerId = playerRepository.getPlayerIdByName(playerName);
    if (!playerId) return null;

    return { playerId, playerSecret };
};

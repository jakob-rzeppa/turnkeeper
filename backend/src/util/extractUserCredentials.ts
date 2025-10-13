import { Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository";

export const extractUserCredentials = (
    socket: Socket
): { playerId: string; playerSecret: string } | null => {
    const playerName = socket.handshake.auth.playerName;
    const playerSecret = socket.handshake.auth.playerSecret;

    if (!playerName || !playerSecret) return null;
    if (typeof playerName !== "string" || typeof playerSecret !== "string")
        return null;

    const playerId = playerRepository.getPlayerIdByName(playerName);
    if (!playerId) return null;

    return { playerId, playerSecret };
};

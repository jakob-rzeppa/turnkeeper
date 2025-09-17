import playerRepository from "../repositories/playerRepository.js";

const arePlayersAlreadyConnected = new Map<string, boolean>();

const isPlayerAlreadyConnected = ({
    playerId,
}: {
    playerId: string;
}): boolean => {
    return arePlayersAlreadyConnected.get(playerId) || false;
};

const setPlayerAlreadyConnected = ({
    playerId,
    connected,
}: {
    playerId: string;
    connected: boolean;
}): void => {
    arePlayersAlreadyConnected.set(playerId, connected);
};

export const authenticatePlayer = ({
    playerId,
    playerSecret,
}: {
    playerId: string;
    playerSecret: string;
}): string => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player) {
        throw new Error("Player not found");
    }

    if (player.secret !== playerSecret) {
        throw new Error("Invalid player credentials");
    }

    if (isPlayerAlreadyConnected({ playerId: player.id })) {
        throw new Error("Player already connected");
    }

    setPlayerAlreadyConnected({ playerId: player.id, connected: true });

    return player.id;
};

export const disconnectPlayer = ({ playerId }: { playerId: string }): void => {
    setPlayerAlreadyConnected({ playerId, connected: false });
};

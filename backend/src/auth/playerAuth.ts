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
    playerName,
    playerSecret,
}: {
    playerName: string;
    playerSecret: string;
}): void => {
    const player = playerRepository.getPlayerByName(playerName);

    if (!player) {
        throw new Error("Player with that name not found");
    }

    if (player.secret !== playerSecret) {
        throw new Error("Invalid player credentials");
    }

    if (isPlayerAlreadyConnected({ playerId: player.id })) {
        throw new Error("Player already connected");
    }

    setPlayerAlreadyConnected({ playerId: player.id, connected: true });
};

export const disconnectPlayer = ({ playerId }: { playerId: string }): void => {
    setPlayerAlreadyConnected({ playerId, connected: false });
};

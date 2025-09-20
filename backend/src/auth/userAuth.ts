import playerRepository from "../repositories/playerRepository.js";

const arePlayersAlreadyConnected = new Map<string, boolean>();

const isUserForPlayerAlreadyConnected = ({
    playerId,
}: {
    playerId: string;
}): boolean => {
    return arePlayersAlreadyConnected.get(playerId) || false;
};

const setConnectedUserForPlayer = ({
    playerId,
    connected,
}: {
    playerId: string;
    connected: boolean;
}): void => {
    arePlayersAlreadyConnected.set(playerId, connected);
};

export const authenticateUser = ({
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

    if (isUserForPlayerAlreadyConnected({ playerId: player.id })) {
        throw new Error("Player already connected");
    }

    setConnectedUserForPlayer({ playerId: player.id, connected: true });

    return player.id;
};

export const disconnectUser = ({ playerId }: { playerId: string }): void => {
    setConnectedUserForPlayer({ playerId, connected: false });
};

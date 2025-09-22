import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";
import { Stat } from "../../types/playerTypes.js";

const sendPlayers = (socket: Socket) => {
    const players = playerRepository.getAllPlayers();
    socket.emit("players", players);
};

const createPlayer = (playerData: { name: string }) => {
    playerRepository.createPlayer(playerData.name);
};

const updatePlayer = ({
    playerId,
    playerData,
}: {
    playerId: string;
    playerData: { name?: string; secret?: string };
}) => {
    playerRepository.updatePlayer(playerId, playerData);
};

const deletePlayer = (playerId: string) => {
    playerRepository.deletePlayer(playerId);
};

const createStatForPlayer = ({
    playerId,
    statData,
}: {
    playerId: string;
    statData: { name: string; value: boolean | number | string | string[] };
}) => {
    playerRepository.createStatForPlayer(playerId, statData);
};

const createStatForAllPlayers = (statData: {
    name: string;
    value: boolean | number | string | string[];
}) => {
    playerRepository.createStatForAllPlayers(statData);
};

const removeStatFromPlayer = ({
    playerId,
    statName,
}: {
    playerId: string;
    statName: string;
}) => {
    playerRepository.removeStatFromPlayer(playerId, statName);
};

export const registerGmPlayersHandler = (socket: Socket) => {
    sendPlayers(socket);

    socket.on("players:create", (playerData) => {
        createPlayer(playerData);
        sendPlayers(socket);
    });

    socket.on("players:update", ({ playerId, playerData }) => {
        updatePlayer({ playerId, playerData });
        sendPlayers(socket);
    });

    socket.on("players:delete", ({ playerId }: { playerId: string }) => {
        deletePlayer(playerId);
        sendPlayers(socket);
    });

    socket.on(
        "players:stats:create",
        ({
            scope,
            playerId,
            statData,
        }: {
            scope: "global" | "player";
            playerId?: string;
            statData: Stat;
        }) => {
            if (scope === "player" && playerId) {
                createStatForPlayer({ playerId, statData });
            } else {
                createStatForAllPlayers(statData);
            }
            sendPlayers(socket);
        }
    );

    socket.on(
        "players:stats:remove",
        ({ playerId, statName }: { playerId: string; statName: string }) => {
            removeStatFromPlayer({ playerId, statName });
            sendPlayers(socket);
        }
    );
};

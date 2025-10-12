import type { StatInterface } from "shared-types";

import { Socket } from "socket.io";

import playerHandler from "../../services/playersHandler.js";

export default class GmPlayersListener {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        this.socket.on("players:create", (playerData: { name: string }) => {
            playerHandler.createPlayer(playerData);
        });

        this.socket.on(
            "players:update",
            ({
                playerData,
                playerId,
            }: {
                playerData: { name: string };
                playerId: string;
            }) => {
                playerHandler.updatePlayer({ playerData, playerId });
            }
        );

        this.socket.on(
            "players:delete",
            ({ playerId }: { playerId: string }) => {
                playerHandler.deletePlayer(playerId);
            }
        );

        this.socket.on(
            "players:stats:create",
            ({
                playerId,
                scope,
                statData,
            }: {
                playerId?: string;
                scope: "global" | "player";
                statData: StatInterface;
            }) => {
                if (scope === "player" && playerId) {
                    playerHandler.createStatForPlayer({ playerId, statData });
                } else {
                    playerHandler.createStatForAllPlayers(statData);
                }
            }
        );

        this.socket.on(
            "players:stats:remove",
            ({
                playerId,
                statName,
            }: {
                playerId: string;
                statName: string;
            }) => {
                playerHandler.removeStatFromPlayer({ playerId, statName });
            }
        );
    }
}

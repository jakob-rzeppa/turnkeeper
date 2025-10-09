import { Socket } from "socket.io";
import { Stat } from "../../types/playerTypes.js";
import playerHandler from "../../services/playersHandler.js";

export default class GmPlayersListener {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        this.socket.on("players:create", (playerData) => {
            playerHandler.createPlayer(playerData);
        });

        this.socket.on("players:update", ({ playerId, playerData }) => {
            playerHandler.updatePlayer({ playerId, playerData });
        });

        this.socket.on(
            "players:delete",
            ({ playerId }: { playerId: string }) => {
                playerHandler.deletePlayer(playerId);
            }
        );

        this.socket.on(
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

import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";
import { Stat } from "../../types/playerTypes.js";

export default class GmPlayersHandler {
    // Singleton instance / register only one GM players handler at a time
    private static instance: GmPlayersHandler | null = null;

    public static registerSocket = (s: Socket) => {
        GmPlayersHandler.instance = new GmPlayersHandler(s);
    };

    public static unregisterSocket = () => {
        GmPlayersHandler.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
        this.socket = s;

        this.sendPlayers();

        this.socket.on("players:create", (playerData) => {
            this.createPlayer(playerData);
            this.sendPlayers();
        });

        this.socket.on("players:update", ({ playerId, playerData }) => {
            this.updatePlayer({ playerId, playerData });
            this.sendPlayers();
        });

        this.socket.on(
            "players:delete",
            ({ playerId }: { playerId: string }) => {
                this.deletePlayer(playerId);
                this.sendPlayers();
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
                    this.createStatForPlayer({ playerId, statData });
                } else {
                    this.createStatForAllPlayers(statData);
                }
                this.sendPlayers();
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
                this.removeStatFromPlayer({ playerId, statName });
                this.sendPlayers();
            }
        );
    }

    private sendPlayers() {
        const players = playerRepository.getAllPlayers();
        this.socket.emit("players", players);
    }

    private createPlayer(playerData: { name: string }) {
        playerRepository.createPlayer(playerData.name);
    }

    private updatePlayer({
        playerId,
        playerData,
    }: {
        playerId: string;
        playerData: { name?: string; secret?: string };
    }) {
        playerRepository.updatePlayer(playerId, playerData);
    }

    private deletePlayer(playerId: string) {
        playerRepository.deletePlayer(playerId);
    }

    private createStatForPlayer({
        playerId,
        statData,
    }: {
        playerId: string;
        statData: { name: string; value: boolean | number | string | string[] };
    }) {
        playerRepository.createStatForPlayer(playerId, statData);
    }

    private createStatForAllPlayers(statData: {
        name: string;
        value: boolean | number | string | string[];
    }) {
        playerRepository.createStatForAllPlayers(statData);
    }

    private removeStatFromPlayer({
        playerId,
        statName,
    }: {
        playerId: string;
        statName: string;
    }) {
        playerRepository.removeStatFromPlayer(playerId, statName);
    }
}

import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";
import playerRepository from "../../repositories/playerRepository.js";

/*
 * Handler for GM to control the game loop (turns, rounds, player order)
 */

export default class GmGameHandler {
    // Singleton instance / register only one GM game handler at a time
    private static instance: GmGameHandler | null = null;

    public static registerSocket = (s: Socket) => {
        GmGameHandler.instance = new GmGameHandler(s);
    };

    public static unregisterSocket = () => {
        GmGameHandler.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
        this.socket = s;

        this.sendTurnInfo();

        this.socket.on("game:turn:next", () => {
            this.nextTurn();
            this.sendTurnInfo();
        });

        this.socket.on(
            "game:init",
            ({ playerIdsInOrder }: { playerIdsInOrder: string[] }) => {
                this.initGameloop(playerIdsInOrder);
                this.sendTurnInfo();
            }
        );
    }

    public sendTurnInfo() {
        const playerOrder = gameloop.getPlayerOrder();
        const playerOrderWithNames = playerOrder.map((id, index) => ({
            id,
            name:
                playerRepository.getPlayerNameById(id) || `Player ${index + 1}`,
        }));

        this.socket.emit("game:turn", {
            playerOrder: playerOrderWithNames,
            isInitialized: gameloop.isInitialized(),
            round: gameloop.getRoundInformation(),
        });
    }

    private initGameloop(playerIdsInOrder: string[]) {
        gameloop.init(playerIdsInOrder);
    }

    private nextTurn() {
        gameloop.nextTurn();
    }
}

import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";

/**
 * Listener for GM actions to control the game loop (turns, rounds, player order)
 */

export default class GmGameListener {
    // Singleton instance / register only one GM game listener at a time
    private static instance: GmGameListener | null = null;

    public static registerSocket = (s: Socket) => {
        GmGameListener.instance = new GmGameListener(s);
    };

    public static unregisterSocket = () => {
        GmGameListener.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
        this.socket = s;

        this.socket.on("game:turn:next", () => {
            this.nextTurn();
        });

        this.socket.on(
            "game:init",
            ({ playerIdsInOrder }: { playerIdsInOrder: string[] }) => {
                this.initGameloop(playerIdsInOrder);
            }
        );

        this.socket.on(
            "game:playerOrder:update",
            ({ playerIdsInOrder }: { playerIdsInOrder: string[] }) => {
                this.updatePlayerOrder(playerIdsInOrder);
            }
        );
    }

    private initGameloop(playerIdsInOrder: string[]) {
        gameloop.init(playerIdsInOrder);
    }

    private updatePlayerOrder(playerIdsInOrder: string[]) {
        gameloop.setPlayerOrder(playerIdsInOrder);
    }

    private nextTurn() {
        gameloop.nextTurn();
    }
}

import { Socket } from "socket.io";

import { gameloop } from "../../services/gameloop.js";

/**
 * Listener for GM actions to control the game loop (turns, rounds, player order)
 */

export default class GmGameListener {
    private socket: Socket;

    public constructor(s: Socket) {
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

        this.socket.on("game:end", () => {
            this.endGame();
        });

        this.socket.on(
            "game:playerOrder:update",
            ({ playerIdsInOrder }: { playerIdsInOrder: string[] }) => {
                this.updatePlayerOrder(playerIdsInOrder);
            }
        );
    }

    private endGame() {
        gameloop.end();
    }

    private initGameloop(playerIdsInOrder: string[]) {
        gameloop.init(playerIdsInOrder);
    }

    private nextTurn() {
        gameloop.nextTurn();
    }

    private updatePlayerOrder(playerIdsInOrder: string[]) {
        gameloop.setPlayerOrder(playerIdsInOrder);
    }
}

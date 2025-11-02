import type { UserGameState } from "./game.js";
import type { UserPlayer } from "./player.js";
export interface BackendToUserPayloads {
    "game:info": {
        gameState: UserGameState | null;
    };
    "player:info": {
        player: UserPlayer;
    };
}
export interface UserToBackendPayloads {
}
//# sourceMappingURL=userEvents.d.ts.map
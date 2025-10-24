import type { GameState } from "./game";
import type { Player } from "./player";
export interface BackendToUserPayloads {
    "game:info": {
        gameState: GameState | null;
    };
    "player:info": {
        player: Omit<Player, "secret">;
    };
}
export interface UserToBackendPayloads {
}
//# sourceMappingURL=userEvents.d.ts.map
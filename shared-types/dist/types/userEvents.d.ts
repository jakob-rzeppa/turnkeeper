import type { UserGameState } from "./game.js";
import type { Message } from "./message.js";
import type { UserPlayer } from "./player.js";
export interface BackendToUserPayloads {
    "game:info": {
        gameState: UserGameState | null;
    };
    "player:info": {
        player: UserPlayer;
    };
    "messages:all": {
        messages: Message[];
    };
}
export interface UserToBackendPayloads {
}
//# sourceMappingURL=userEvents.d.ts.map
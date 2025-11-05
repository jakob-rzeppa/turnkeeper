import type { UserGameState } from "./game.js";
import type { Message } from "./message.js";
import type { UserPlayer } from "./player.js";
export interface BackendToUserEventPayloads {
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
export interface UserToBackendEventPayloads {
    "messages:send": {
        content: string;
    };
}
//# sourceMappingURL=userEvents.d.ts.map
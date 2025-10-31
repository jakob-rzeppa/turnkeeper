import type { GameState } from "./game.js";
import type { UserPlayer } from "./player.js";

export interface BackendToUserPayloads {
    "game:info": { gameState: GameState | null };
    "player:info": { player: UserPlayer };
}

export interface UserToBackendPayloads {}

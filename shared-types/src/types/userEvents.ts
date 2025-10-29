import type { GameState } from "./game";
import type { UserPlayer } from "./player";

export interface BackendToUserPayloads {
    "game:info": { gameState: GameState | null };
    "player:info": { player: UserPlayer };
}

export interface UserToBackendPayloads {}

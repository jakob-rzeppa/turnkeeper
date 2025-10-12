import type { GameState } from "./game";
import type { Player } from "./player";

export interface BackendToUserPayloads {
    "game:info": GameState;
    "player:info": Omit<Player, "secret">;
}

export interface UserToBackendPayloads {}

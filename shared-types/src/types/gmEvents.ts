import type { GameState } from "./game";
import type { LogEntry } from "./log";
import type { Player, PlayerStat } from "./player";

export interface BackendToGmEventPayloads {
    "game:info": GameState;
    "log:entry": { entry: LogEntry };
    "players:info": { players: Player[] };
}
export interface GmToBackendEventPayloads {
    "game:init": {
        playerIdsInOrder: string[];
    };
    "game:playerOrder:update": {
        playerIdsInOrder: string[];
    };
    "players:create": {
        name: string;
    };
    "players:update": {
        playerId: string;
        playerData: Partial<Omit<Player, "id">>;
    };
    "players:delete": {
        playerId: string;
    };
    "players:stats:create": {
        scope: "global" | "player";
        playerId?: string;
        statData: PlayerStat;
    };
    "players:stats:remove": {
        playerId: string;
        statName: string;
    };
}

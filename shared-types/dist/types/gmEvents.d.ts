import type { GameState } from "./game.js";
import type { LogEntry } from "./log.js";
import type { Player, PlayerStat } from "./player.js";
export interface BackendToGmEventPayloads {
    "game:info": {
        gameState: GameState | null;
    };
    "log:entry": {
        entry: LogEntry;
    };
    "players:info": {
        players: Player[];
    };
}
export interface GmToBackendEventPayloads {
    "game:init": {
        playerIdsInOrder: number[];
    };
    "game:playerOrder:update": {
        playerIdsInOrder: number[];
    };
    "players:create": {
        name: string;
    };
    "players:update": {
        playerId: number;
        playerData: Partial<Omit<Player, "id" | "stats">>;
    };
    "players:delete": {
        playerId: number;
    };
    "players:stats:create": {
        scope: "global" | "player";
        playerId?: number;
        statData: Omit<PlayerStat, "id">;
    };
    "players:stats:update": {
        playerId: number;
        statId: number;
        name?: PlayerStat["name"];
        value?: PlayerStat["value"];
    };
    "players:stats:remove": {
        playerId: number;
        statId: number;
    };
}
//# sourceMappingURL=gmEvents.d.ts.map
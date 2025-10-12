import type { PlayerInterface } from "./player";
export interface GameStateInterface {
    isInitialized: boolean;
    round: {
        roundNumber: number;
    };
    currentPlayerId: string | null;
    playerOrder: PlayerInterface[];
}
//# sourceMappingURL=game.d.ts.map
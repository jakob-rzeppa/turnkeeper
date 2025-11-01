import type { Player } from "./player.js";
export type PlayerOrderWithNames = Pick<Player, "id" | "name">[];
export interface GameState {
    id: number;
    playerOrder: PlayerOrderWithNames;
    currentPlayerIndex: number;
    roundNumber: number;
    notes: string;
    hiddenNotes: string;
}
export type UserGameState = Pick<GameState, "id" | "playerOrder" | "currentPlayerIndex" | "roundNumber" | "notes">;
//# sourceMappingURL=game.d.ts.map
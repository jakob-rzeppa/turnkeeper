import type { Player } from "./player.js";
export type PlayerOrderWithNames = Pick<Player, "id" | "name">[];
export interface GameState {
    id: number;
    playerOrder: PlayerOrderWithNames;
    currentPlayerIndex: number;
    roundNumber: number;
}
//# sourceMappingURL=game.d.ts.map
import type { Player } from "./player";

export interface RoundInformation {
    currentPlayerIndex: number;
    roundNumber: number;
}

export type PlayerOrderWithNames = Pick<Player, "id" | "name">[];

export interface GameState {
    isInitialized: boolean;
    round: RoundInformation;
    playerOrder: PlayerOrderWithNames;
}

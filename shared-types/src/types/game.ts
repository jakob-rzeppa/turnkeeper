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

// The UserGameState type only exposes non-sensitive information and is used for game state representation in the user interface.
export type UserGameState = Pick<
    GameState,
    "id" | "playerOrder" | "currentPlayerIndex" | "roundNumber" | "notes"
>;

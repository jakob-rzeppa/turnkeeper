export interface GameState {
    id: number;
    playerOrder: number[];
    currentPlayerIndex: number;
    roundNumber: number;
    notes: string;
    hiddenNotes: string;
}

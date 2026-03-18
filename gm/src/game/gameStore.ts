import { defineStore } from 'pinia';
import { ref } from 'vue';

export type Stat = {
    id: string;
    key: string;
    valueType: 'number' | 'string' | 'boolean';
    stringValue: string | null;
    numberValue: number | null;
    booleanValue: boolean | null;
};

export type Tradable = {
    id: string;
    name: string;
    value: number;
};

export type Player = {
    id: string;
    userId: string | null;
    stats: Stat[];
    tradables: Tradable[];
};

export type Game = {
    id: string;
    name: string;

    players: Player[];

    roundNumber: number;
    currentPlayerIndex: number;

    notes: string;
    hiddenNotes: string;
};

export const useGameStore = defineStore('game', () => {
    const game = ref<Game | null>(null);

    const setGame = (newGame: Game) => {
        game.value = newGame;
    };

    return { game, setGame };
});

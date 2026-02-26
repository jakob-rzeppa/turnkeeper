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

export type PlayerUser = {
    id: string;
    name: string;
};

export type Player = {
    id: string;
    user: PlayerUser | null;
    stats: Stat[];
};

export type Game = {
    id: string;
    name: string;

    players: Player[];

    roundNumber: number;
    currentPlayerIndex: number;
};

export const useGameStore = defineStore('game', () => {
    const game = ref<Game | null>(null);

    const setGame = (newGame: Game) => {
        game.value = newGame;
    };

    return { game, setGame };
});

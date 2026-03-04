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

export type OwnPlayer = {
    id: string;
    userId: string;
    stats: Stat[];
};

export type Player = {
    id: string;
    userId: string | null;
};

export type Game = {
    id: string;
    name: string;

    ownPlayer: OwnPlayer | null;
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

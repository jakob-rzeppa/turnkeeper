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

export type OwnPlayer = {
    id: string;
    userId: string;
    stats: Stat[];
    tradables: Tradable[];
};

export type Player = {
    id: string;
    userId: string | null;
};

export type Game = {
    id: string;
    name: string;
    gm_user_id: string;

    ownPlayer: OwnPlayer | null;
    players: Player[];

    roundNumber: number;
    currentPlayerIndex: number;

    notes: string;
};

export const useGameStore = defineStore('game', () => {
    const game = ref<Game | null>(null);

    const setGame = (newGame: Game) => {
        game.value = newGame;
    };

    return { game, setGame };
});

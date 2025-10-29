import { defineStore } from 'pinia';
import { markRaw, type Component } from 'vue';

/*
 * This Store manages the open Modals in the application.
 * It keeps track of the open modals in a stack.
 */

interface Modal {
    id: number;
    component: Component;
    props?: Record<string, unknown>;
    width: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '4xl' | '6xl' | '8xl';
}

// incremental id for new modals
let id = 0;

export const useModalStore = defineStore('modal', {
    state: () => ({
        modals: [] as Modal[],
    }),
    actions: {
        openModal(
            component: Component,
            props: Modal['props'] = {},
            width: Modal['width'] = '2xl',
        ): void {
            this.modals.push({ id: ++id, component: markRaw(component), props, width });
        },
        closeTopModal(): void {
            if (this.modals.length > 0) {
                this.modals.pop();
            }
        },
    },
    getters: {
        topModal: (state): Modal | null => {
            return state.modals.length > 0 ? state.modals[state.modals.length - 1] : null;
        },
    },
});

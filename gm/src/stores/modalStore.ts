import { defineStore } from 'pinia'
import type { ShallowRef } from 'vue'

interface Modal {
    id: number
    component: ShallowRef<any>
    props?: Record<string, unknown>
}

// incremental id for new modals
let id = 0

export const useModalStore = defineStore('modal', {
    state: () => ({
        modals: [] as Modal[],
    }),
    actions: {
        openModal(component: ShallowRef<any>, props: Modal['props'] = {}): void {
            this.modals.push({ id: ++id, component, props })
        },
        closeTopModal(): void {
            if (this.modals.length > 0) {
                this.modals.pop()
            }
        },
    },
    getters: {
        topModal: (state): Modal | null => {
            return state.modals.length > 0 ? state.modals[state.modals.length - 1] : null
        },
    },
})

import { defineStore } from 'pinia'

interface Modal {
    id: number
    component: any
    props?: Record<string, unknown>
}

// incremental id for new modals
let id = 0

export const useModalStore = defineStore('modal', {
    state: () => ({
        modals: [] as Modal[],
    }),
    actions: {
        openModal(component: any, props: Modal['props'] = {}): void {
            this.modals.push({ id: ++id, component, props })
        },
        closeTopModal(): void {
            if (this.modals.length > 0) {
                this.modals.pop()
            }
        },
    },
})

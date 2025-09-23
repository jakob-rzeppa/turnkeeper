import type { LogEntry } from '@/types/logTypes'
import { defineStore } from 'pinia'

export const useLogStore = defineStore('log', {
    state: () => ({
        logs: [] as LogEntry[],
    }),
})

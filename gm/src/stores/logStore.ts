import useConnection from '@/composables/connection'
import type { LogEntry } from '@/types/logTypes'
import { defineStore } from 'pinia'
import { ref } from 'vue'

const { socket } = useConnection()

export const useLogStore = defineStore('log', () => {
    // The store shall only be modified by events from the backend.
    const logs = ref([] as LogEntry[])

    socket.on('log', (logEntry) => {
        logs.value.push(logEntry)
    })

    return {
        logs,
    }
})

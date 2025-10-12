import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type PlayerInterface } from 'shared-types'

export const usePlayerStore = defineStore('player', () => {
    const player = ref<PlayerInterface | null>(null)

    const connection = useConnection()

    connection.socket.on('player', (newPlayer) => {
        player.value = newPlayer || null
    })

    return { player }
})

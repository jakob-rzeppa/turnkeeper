import useConnection from '@/composables/connection'
import type { Player } from '@/types/player'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const usePlayerStore = defineStore('player', () => {
    const player = ref<Player | null>(null)

    const connection = useConnection()

    connection.socket.on('player', (newPlayer) => {
        player.value = newPlayer || null
    })

    return { player }
})

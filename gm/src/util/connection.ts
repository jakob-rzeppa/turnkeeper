import { usePlayerStore } from '@/stores/playerStore'
import io from 'socket.io-client'
import { onMounted, ref } from 'vue'

export const socket = io('http://localhost:3000/gm', { autoConnect: false })

const connected = ref(false)

onMounted(() => {
    const playerStore = usePlayerStore()

    socket.onAny((event, ...args) => {
        console.log(event, args)
    })

    socket.on('players', (players) => {
        console.log('Received players:', players)

        playerStore.players = players
    })
})

export default {
    connect: () => {
        try {
            socket.auth = { gmConnection: true }
            socket.connect()
            connected.value = true
        } catch (error) {
            console.error('Error connecting:', error)
        }
    },
    disconnect: () => {
        try {
            socket.disconnect()
            connected.value = false
        } catch (error) {
            console.error('Error disconnecting:', error)
        }
    },
    isConnected: () => {
        return connected
    },
}

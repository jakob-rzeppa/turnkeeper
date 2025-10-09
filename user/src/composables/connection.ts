import io from 'socket.io-client'
import { ref } from 'vue'

const socket = io('http://localhost:3000/user', { autoConnect: false })
const isConnected = ref(socket.connected)

socket.on('connect', () => {
    isConnected.value = true
})

socket.on('disconnect', () => {
    isConnected.value = false
})

socket.on('connect_error', (err) => {
    console.error(err.message)
    isConnected.value = false
})

socket.onAny((event, ...args) => {
    console.log(event, args)
})

export default function useConnection() {
    function connect({ playerName, playerSecret }: { playerName: string; playerSecret: string }) {
        if (!socket.connected) {
            socket.auth = { playerName, playerSecret }
            socket.connect()
        }
    }

    function disconnect() {
        if (socket.connected) {
            socket.disconnect()
        }
    }

    return {
        socket,
        isConnected,
        connect,
        disconnect,
    }
}

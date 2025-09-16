import io from 'socket.io-client'
import { ref } from 'vue'

export const socket = io('http://localhost:3000/gm', { autoConnect: false })

socket.on('connect_error', (err) => {
    console.error(err.message)
})

const connected = ref(false)

socket.onAny((event, ...args) => {
    console.log(event, args)
})

export default {
    connect: () => {
        try {
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

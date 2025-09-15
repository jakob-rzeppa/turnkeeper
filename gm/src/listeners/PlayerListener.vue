<script setup lang="ts">
import { socket } from '../connectionHandlers/connection'
import { usePlayerStore } from '@/stores/playerStore'

const playerStore = usePlayerStore()

socket.on('players', (players) => {
    console.log('Received players in connection handler:', players)
    playerStore.players = players
})
</script>

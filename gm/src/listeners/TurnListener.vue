<script setup lang="ts">
import { useTurnStore } from '@/stores/turnStore'
import { socket } from '../util/connection'

const turnStore = useTurnStore()

socket.on(
    'game:turn',
    ({
        playerOrder,
        round,
    }: {
        playerOrder: { id: string; name: string }[]
        round: { roundNumber: number; currentPlayerIndex: number }
    }) => {
        turnStore.playerOrder = playerOrder
        turnStore.round = {
            roundNumber: round.roundNumber,
            currentPlayerIndex: round.currentPlayerIndex,
        }
    },
)
</script>

<template></template>

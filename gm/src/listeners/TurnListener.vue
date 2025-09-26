<script setup lang="ts">
import { useTurnStore } from '@/stores/turnStore'
import { socket } from '../util/connection'

const turnStore = useTurnStore()

socket.on(
    'game:turn',
    ({
        playerOrder,
        round,
        isInitialized,
    }: {
        playerOrder: { id: string; name: string }[]
        round: { roundNumber: number; currentPlayerIndex: number }
        isInitialized: boolean
    }) => {
        turnStore.playerOrder = playerOrder
        turnStore.round = {
            roundNumber: round.roundNumber,
            currentPlayerIndex: round.currentPlayerIndex,
        }
        turnStore.isInitialized = isInitialized
    },
)
</script>

<template></template>

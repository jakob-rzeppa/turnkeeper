<script setup lang="ts">
import { ref, watch } from 'vue'

import type { Player } from '@/types/player'

const { player } = defineProps<{ player: Player | null }>()

const isVisible = ref(false)

const playerNameRef = ref(null as null | string)
const playerSecretRef = ref(null as null | string)

const openEditor = (newPlayer: Player | null) => {
    if (newPlayer) {
        playerNameRef.value = newPlayer.name
        playerSecretRef.value = newPlayer.secret
        isVisible.value = true
        return
    }

    playerNameRef.value = null
    playerSecretRef.value = null
    isVisible.value = false
}
watch(() => player, openEditor, { immediate: true })
</script>

<template>
    <div :hidden="!isVisible">
        <input v-model="playerNameRef" placeholder="Player Name" />
        <input v-model="playerSecretRef" placeholder="Player Secret" />
    </div>
</template>

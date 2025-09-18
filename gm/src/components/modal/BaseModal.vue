<script setup lang="ts">
defineProps<{ zIndex: number }>()
const emit = defineEmits(['close'])
</script>

<template>
    <div :style="{ zIndex }">
        <div @click="emit('close')" />
        <div>
            <slot />
        </div>
    </div>
</template>

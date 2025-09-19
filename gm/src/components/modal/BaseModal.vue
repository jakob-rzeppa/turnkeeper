<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

const props = defineProps<{ zIndex: number }>()

const emit = defineEmits(['close'])

/*
 * A base modal component that provides a backdrop and centers its content.
 * It emits a 'close' event when the backdrop is clicked or escape is pressed.
 */

function handleEscape(event: KeyboardEvent) {
    if (event.key === 'Escape' || event.key === 'Esc') {
        event.preventDefault()
        emit('close')
    }
}
onMounted(() => document.addEventListener('keydown', handleEscape))
onUnmounted(() => document.removeEventListener('keydown', handleEscape))
</script>

<template>
    <div :style="{ zIndex: props.zIndex }" class="fixed w-screen h-screen top-0 left-0">
        <div @click="emit('close')" class="w-full h-full backdrop-blur-sm" />
        <div
            class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-4 rounded-lg shadow-lg bg-base-200 min-w-1/2 min-h-1/2"
        >
            <div class="flex flex-col gap-4 justify-center h-full">
                <slot />
            </div>
        </div>
    </div>
</template>

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
    <div
        :style="{ zIndex: props.zIndex }"
        class="fixed inset-0 flex items-center justify-center p-4 animate-in fade-in duration-200"
    >
        <!-- Backdrop -->
        <div
            @click="emit('close')"
            class="absolute inset-0 bg-gradient-to-br from-base-200/60 via-base-200/40 to-base-200/60 backdrop-blur-md"
        />
        <div
            class="relative bg-base-100 rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-hidden animate-in zoom-in-95 duration-200"
        >
            <div class="p-6 overflow-y-auto max-h-[90vh]">
                <div class="space-y-6">
                    <slot />
                </div>
            </div>
        </div>
    </div>
</template>

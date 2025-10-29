<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';

const props = defineProps<{
    zIndex: number;
    width: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '4xl' | '6xl' | '8xl';
}>();

const emit = defineEmits(['close']);

/*
 * A base modal component that provides a backdrop and centers its content.
 * It emits a 'close' event when the backdrop is clicked or escape is pressed.
 */

function handleEscape(event: KeyboardEvent) {
    if (event.key === 'Escape' || event.key === 'Esc') {
        event.preventDefault();
        emit('close');
    }
}
onMounted(() => document.addEventListener('keydown', handleEscape));
onUnmounted(() => document.removeEventListener('keydown', handleEscape));
</script>

<template>
    <div
        :style="{ zIndex: props.zIndex }"
        class="fixed inset-0 flex items-center justify-center p-4 animate-in fade-in duration-200"
    >
        <!-- Backdrop -->
        <div
            @click="emit('close')"
            class="absolute inset-0 bg-linear-to-br from-base-200/60 via-base-200/40 to-base-200/60 backdrop-blur-md"
        />
        <div
            :class="{
                ['relative bg-base-100 rounded-xl shadow-2xl w-full max-h-[90vh] overflow-hidden animate-in zoom-in-95 duration-200']: true,
                'max-w-sm': props.width === 'sm',
                'max-w-md': props.width === 'md',
                'max-w-lg': props.width === 'lg',
                'max-w-xl': props.width === 'xl',
                'max-w-2xl': props.width === '2xl',
                'max-w-4xl': props.width === '4xl',
                'max-w-6xl': props.width === '6xl',
                'max-w-8xl': props.width === '8xl',
            }"
        >
            <div class="p-6 overflow-y-auto max-h-[90vh]">
                <div class="space-y-6">
                    <slot />
                </div>
            </div>
        </div>
    </div>
</template>

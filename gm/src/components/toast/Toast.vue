<script setup lang="ts">
const props = defineProps<{
    message: string;
    type: 'info' | 'success' | 'warning' | 'error';
    duration: number | null;
}>();
</script>

<template>
    <div
        :class="{
            'relative overflow-hidden rounded-lg shadow-lg max-w-sm w-full border': true,
            'bg-info text-info-content border-info-content': props.type === 'info',
            'bg-success text-success-content border-success-content': props.type === 'success',
            'bg-warning text-warning-content border-warning-content': props.type === 'warning',
            'bg-error text-error-content border-error-content': props.type === 'error',
        }"
    >
        <div class="px-4 py-3 font-medium" title="Click to dismiss">
            {{ props.message }}
        </div>
        <div
            v-if="props.duration"
            :class="{
                'h-1 w-full origin-left animate-progress': true,
                'bg-info-content': props.type === 'info',
                'bg-success-content': props.type === 'success',
                'bg-warning-content': props.type === 'warning',
                'bg-error-content': props.type === 'error',
            }"
            :style="{ animationDuration: `${props.duration}ms` }"
        ></div>
    </div>
</template>

<style scoped>
@keyframes progress {
    from {
        transform: scaleX(1);
    }
    to {
        transform: scaleX(0);
    }
}

.animate-progress {
    animation: progress linear forwards;
}
</style>

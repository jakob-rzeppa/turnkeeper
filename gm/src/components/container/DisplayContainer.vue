<script setup lang="ts">
const props = defineProps<{
    label?: string
}>()
</script>

<template>
    <div class="card bg-base-100 shadow-lg border border-base-300 h-full">
        <div class="card-body p-6">
            <h2 v-if="props.label" class="card-title text-primary mb-4 flex items-center">
                {{ props.label }}
            </h2>
            <div class="flex-1">
                <slot />
            </div>
        </div>
    </div>
</template>

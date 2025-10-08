<script setup lang="ts">
const props = defineProps<{
    label?: string
}>()
</script>

<template>
    <fieldset class="w-full fieldset bg-base-200 border-base-300 rounded-box border p-4">
        <legend v-if="props.label" class="fieldset-legend">{{ props.label }}</legend>
        <slot />
    </fieldset>
</template>

<script setup lang="ts">
import { useLogStore } from '@/stores/logStore'

const logStore = useLogStore()
</script>

<template>
    <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4">
        <legend class="fieldset-legend">Logs</legend>
        <ul>
            <li v-for="(log, index) in logStore.logs" :key="index">
                <strong>{{ log.timestamp }} [{{ log.severity }}]</strong>: {{ log.message }}
                <span v-if="log.details">| </span>
                <span v-if="log.details">Details: {{ log.details }}</span>
            </li>
        </ul>
    </fieldset>
</template>

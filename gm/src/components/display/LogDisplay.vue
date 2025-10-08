<script setup lang="ts">
import { useLogStore } from '@/stores/logStore'
import DisplayContainer from '../DisplayContainer.vue'

const logStore = useLogStore()
</script>

<template>
    <DisplayContainer label="Logs">
        <ul>
            <li v-for="(log, index) in logStore.logs" :key="index">
                <strong>{{ new Date(log.timestamp).toLocaleString() }} [{{ log.severity }}]</strong
                >:
                {{ log.message }}
                <span v-if="log.details">| </span>
                <span v-if="log.details">Details: {{ log.details }}</span>
            </li>
        </ul>
    </DisplayContainer>
</template>

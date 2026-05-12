<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useGameEditor } from './useGameEditor';

const route = useRoute();

const gameEditor = useGameEditor();
</script>

<template>
    <div v-if="gameEditor.isSaved.value !== true">
        <p class="alert alert-warning">You have unsaved changes.</p>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'uninitialized'">
        <h2 class="text-xl font-bold mb-4">Uninitialized</h2>
        <button
            class="btn btn-primary"
            @click="gameEditor.checkSourceCode(route.params.id as string)"
        >
            Check Code
        </button>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'loading'">
        <h2 class="text-xl font-bold mb-4">Checking...</h2>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'error'">
        <h2 class="text-xl font-bold mb-4">Error</h2>
        <p>{{ gameEditor.checkResult.value.error }}</p>
        <button
            class="btn btn-primary"
            @click="gameEditor.checkSourceCode(route.params.id as string)"
        >
            Retry
        </button>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'success'">
        <h2 class="text-xl font-bold mb-4">Check Result</h2>
        <div v-if="gameEditor.checkResult.value.data.is_valid">
            <p class="text-green-600">Code is valid!</p>
            <pre>
                {{ gameEditor.checkResult.value.data.output }}
            </pre>
        </div>
        <div v-else>
            <p class="text-red-600">Code is invalid.</p>
            <pre>
                {{ gameEditor.checkResult.value.data.errors }}
            </pre>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useLogStore } from '@/stores/logStore'
import DisplayContainer from '../container/DisplayContainer.vue'

const logStore = useLogStore()
</script>

<template>
    <DisplayContainer label="Game Logs">
        <div class="h-96 overflow-y-auto">
            <div v-if="logStore.logs.length > 0" class="space-y-2">
                <div
                    v-for="(log, index) in logStore.logs"
                    :key="index"
                    class="p-3 rounded-lg border-l-4 transition-all hover:shadow-sm"
                    :class="{
                        'bg-info/10 border-l-info': log.severity === 'info',
                        'bg-warning/10 border-l-warning': log.severity === 'warning',
                        'bg-error/10 border-l-error': log.severity === 'error',
                        'bg-base-200 border-l-base-content/20': ![
                            'info',
                            'warning',
                            'error',
                        ].includes(log.severity),
                    }"
                >
                    <div class="flex items-start justify-between gap-3">
                        <div class="flex-1 min-w-0">
                            <div class="flex items-center gap-2 mb-1">
                                <div
                                    class="badge badge-sm"
                                    :class="{
                                        'badge-info': log.severity === 'info',
                                        'badge-warning': log.severity === 'warning',
                                        'badge-error': log.severity === 'error',
                                        'badge-ghost': !['info', 'warning', 'error'].includes(
                                            log.severity,
                                        ),
                                    }"
                                >
                                    {{ log.severity }}
                                </div>
                                <time class="text-xs text-base-content/60">
                                    {{ new Date(log.timestamp).toLocaleString() }}
                                </time>
                            </div>
                            <p class="text-sm font-medium text-base-content">{{ log.message }}</p>
                            <p
                                v-if="log.details"
                                class="text-xs text-base-content/70 mt-1 font-mono bg-base-300 p-2 rounded"
                            >
                                {{ log.details }}
                            </p>
                        </div>

                        <!-- Severity Icon -->
                        <div class="flex-shrink-0">
                            <svg
                                v-if="log.severity === 'info'"
                                class="w-4 h-4 text-info"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                            <svg
                                v-else-if="log.severity === 'warning'"
                                class="w-4 h-4 text-warning"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                            <svg
                                v-else-if="log.severity === 'error'"
                                class="w-4 h-4 text-error"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                            <svg
                                v-else
                                class="w-4 h-4 text-base-content/50"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                            <svg
                                v-else
                                class="w-4 h-4 text-base-content/50"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            <div v-else class="flex flex-col items-center justify-center h-full py-8">
                <svg
                    class="w-12 h-12 text-base-content/20 mb-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    ></path>
                </svg>
                <div class="text-center">
                    <h3 class="text-sm font-medium text-base-content/70">No logs yet</h3>
                    <p class="text-xs text-base-content/50 mt-1">Game activity will appear here</p>
                </div>
            </div>
        </div>

        <!-- Auto-scroll to bottom indicator -->
        <div v-if="logStore.logs.length > 5" class="mt-3 text-xs text-center text-base-content/50">
            <kbd class="kbd kbd-xs">Scroll</kbd> to view older logs
        </div>
    </DisplayContainer>
</template>

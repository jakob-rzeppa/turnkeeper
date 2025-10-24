<script setup lang="ts">
import useConnection from '@/composables/useConnection';

const { isConnected, connect, disconnect } = useConnection();
</script>

<template>
    <div class="flex flex-col gap-2">
        <div class="text-xs text-center text-base-content/60">
            Status:
            <span :class="isConnected ? 'text-success' : 'text-error'">
                {{ isConnected ? 'Connected' : 'Disconnected' }}
            </span>
        </div>
        <button v-if="!isConnected" class="btn btn-success btn-sm w-full" @click="connect">
            Connect
        </button>
        <button v-if="isConnected" class="btn btn-error btn-sm w-full" @click="disconnect">
            Disconnect
        </button>
    </div>
</template>

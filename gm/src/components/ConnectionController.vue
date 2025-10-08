<script setup lang="ts">
import useConnection from '@/composables/connection'

const { isConnected, connect, disconnect } = useConnection()
</script>

<template>
    <div>
        <button v-if="!isConnected" class="btn btn-primary btn-sm w-fit" @click="connect">
            Connect
        </button>
        <button v-if="isConnected" class="btn btn-error btn-sm w-fit" @click="disconnect">
            Disconnect
        </button>
    </div>
</template>

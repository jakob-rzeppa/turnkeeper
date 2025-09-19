<script setup lang="ts">
import connection from '@/util/connection'

const isConnected = connection.isConnected()

function connect() {
    connection.connect()
}

function disconnect() {
    connection.disconnect()
}
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

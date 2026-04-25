<script setup lang="ts">
import { useWsConnection } from '../../api/useWsConnection';

const wsConnection = useWsConnection();
</script>

<template>
    <button class="btn btn-error" @click="wsConnection.disconnect()">Disconnect</button>
</template>

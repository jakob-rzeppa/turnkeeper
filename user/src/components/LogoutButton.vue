<script setup lang="ts">
import useConnection from '@/composables/useConnection';

const connection = useConnection();

function logout() {
    connection.disconnect();
}
</script>

<template>
    <div class="flex align-middle justify-center">
        <button class="btn btn-accent" @click="logout">Logout</button>
    </div>
</template>

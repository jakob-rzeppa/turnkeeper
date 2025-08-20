<script setup lang="ts">
import { onMounted } from 'vue';
import { socket } from './socket';

onMounted(() => {
  socket.auth = { username: 'jakob' };

  socket.connect();
});

function sendMessage() {
  socket.send('hi');
}
</script>

<template>
  <h1>Hello</h1>
  <button @click="sendMessage">Send Message</button>
</template>

<style scoped></style>

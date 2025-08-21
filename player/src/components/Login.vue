<script setup lang="ts">
import { ref } from 'vue'

import connectionService from '@/util/connectionService'

const name = ref('')

function login() {
  connectionService.connect(name.value)
}
</script>

<template>
  <div>
    <h1>Login</h1>
    <input v-model="name" placeholder="Enter the name of your character" />
    <button @click="login">Login</button>
  </div>
</template>

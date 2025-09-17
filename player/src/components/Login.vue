<script setup lang="ts">
import { ref } from 'vue'

import connectionService from '@/util/connection'

const name = ref('')
const secret = ref('')

function login() {
  connectionService.connect({ playerName: name.value, playerSecret: secret.value })
}
</script>

<template>
  <div>
    <h1>Login</h1>
    <input v-model="name" placeholder="Name" />
    <input v-model="secret" placeholder="Secret" />
    <button @click="login">Login</button>
  </div>
</template>

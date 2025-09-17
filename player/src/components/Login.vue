<script setup lang="ts">
import { ref } from 'vue'

import connectionService from '@/util/connection'

function login() {
  connectionService.connect()
}
</script>

<template>
  <div>
    <h1>Login</h1>
    <button @click="login">Login</button>
  </div>
</template>

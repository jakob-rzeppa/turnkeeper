<script setup lang="ts">
import { ref } from 'vue'

import connectionService from '@/util/connectionService'

const name = ref('')
</script>

<template>
  <button @click="() => connectionService.connect()">Login</button>
</template>

<script setup lang="ts">
import useConnection from '@/composables/connection'
import { ref } from 'vue'

const connection = useConnection()

const name = ref('')
const secret = ref('')

function login() {
    connection.connect({ playerName: name.value, playerSecret: secret.value })
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

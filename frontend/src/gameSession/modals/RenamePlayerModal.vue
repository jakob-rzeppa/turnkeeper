<script setup lang="ts">
import { ref } from 'vue';
import { useCommandEmitter } from '../useCommandEmitter';

const props = defineProps<{
    player: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();

const commandEmitter = useCommandEmitter();

const newName = ref(props.player);

const renamePlayer = () => {
    if (newName.value.trim() && newName.value !== props.player) {
        commandEmitter.changePlayerName(props.player, newName.value.trim());
        emit('close');
    }
};
</script>

<template>
    <form @submit.prevent="renamePlayer" class="flex flex-col gap-4">
        <h2 class="text-xl">Rename Player</h2>
        <input
            v-model="newName"
            type="text"
            placeholder="Enter new player name"
            class="input input-bordered w-full"
        />
        <button type="submit" class="btn btn-primary w-full" :disabled="!newName.trim() || newName === props.player">
            Rename Player
        </button>
    </form>
</template>
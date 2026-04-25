<script setup lang="ts">
import { computed, ref } from 'vue';
import { useCommandEmitter } from '../../commands/useCommandEmitter';
import { useUsersStore } from '../../users/usersStore';
import { useGameStore } from '../gameStore';

const props = defineProps<{
    playerId: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();

const commandEmitter = useCommandEmitter();
const gameStore = useGameStore();
const usersStore = useUsersStore();

const selectedUserId = ref<string | null>(null);

const availableUsers = computed(() => {
    const allUsers = usersStore.users;
    // Filter out users already attached to players
    const attachedUserIds = new Set(
        gameStore.game?.players.filter(p => p.userId).map(p => p.userId as string) || []
    );
    return allUsers.filter(u => !attachedUserIds.has(u.id));
});

const attachUserToPlayer = () => {
    if (selectedUserId.value) {
        commandEmitter.attachUserToPlayer(selectedUserId.value, props.playerId);
        selectedUserId.value = null;
        emit('close');
    }
};
</script>

<template>
    <div>
        <form @submit.prevent="attachUserToPlayer" class="flex flex-col gap-4">
            <h2 class="text-xl">Attach a user to the player</h2>
            <select v-model="selectedUserId" class="select select-bordered w-full">
                <option disabled value="">Select a user to attach</option>
                <option v-for="user in availableUsers" :key="user.id" :value="user.id">
                    {{ user.name }}
                </option>
            </select>
            <button type="submit" class="btn btn-primary w-full" :disabled="!selectedUserId">
                Attach User
            </button>
        </form>
    </div>
</template>

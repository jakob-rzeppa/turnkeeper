<script setup lang="ts">
import { computed, ref } from 'vue';
import { useUsersStore } from '../../users/usersStore';
import { useCommandEmitter } from '../useCommandEmitter';
import { useSession } from '../useSession';

const props = defineProps<{
    player: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();

const commandEmitter = useCommandEmitter();
const session = useSession();
const usersStore = useUsersStore();

const selectedUserId = ref<string | null>(null);

const availableUsers = computed(() => {
    const allUsers = usersStore.users;
    // Filter out users already attached to players
    const attachedUserIds = new Set(
        session.gameState.value?.players.filter(p => p.user_id).map(p => p.user_id as string) || []
    );
    return allUsers.filter(u => !attachedUserIds.has(u.id));
});

const attachUserToPlayer = () => {
    if (selectedUserId.value) {
        commandEmitter.attachUserToPlayer(props.player, selectedUserId.value);
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

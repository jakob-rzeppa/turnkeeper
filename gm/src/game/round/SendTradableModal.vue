<script setup lang="ts">
import { ref } from 'vue';
import { useGameStore } from '../gameStore';
import { useUsersStore } from '../../users/usersStore';
import { useEventEmitter } from '../../events/useEventEmitter';

const props = defineProps<{
    tradableId: string;
    tradableName: string;
    fromPlayerId: string;
    playerTradableValue: number;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();

const gameStore = useGameStore();
const usersStore = useUsersStore();
const eventEmitter = useEventEmitter();

const toPlayerId = ref<string | null>(null);
const tradableSendAmount = ref(0);

const sendTradableFromPlayerToPlayer = () => {
    if (!toPlayerId.value) {
        alert('Please select both a sender and a receiver.');
        return;
    }
    eventEmitter.sendTradable(
        props.fromPlayerId,
        toPlayerId.value,
        props.tradableId,
        tradableSendAmount.value
    );
    emit('close');
};
</script>

<template>
    <div class="space-y-4">
        <div class="space-y-2">
            <h2 class="text-2xl font-bold">Send {{ props.tradableName }}</h2>
            <div class="flex flex-wrap items-center gap-2 text-sm text-base-content/70">
                <span>From</span>
                <span class="badge badge-outline">
                    {{ usersStore.getPlayerName(props.fromPlayerId) }}
                </span>
                <span>Available</span>
                <span class="badge badge-primary badge-outline">
                    {{ props.playerTradableValue }}
                </span>
            </div>
        </div>

        <form class="flex flex-col gap-2" @submit.prevent="sendTradableFromPlayerToPlayer">
            <label class="select select-bordered w-full">
                <span class="label">To</span>
                <select v-model="toPlayerId">
                    <option disabled value="">Select player</option>
                    <option
                        v-for="player in gameStore.game?.players.filter(
                            p => p.id !== props.fromPlayerId
                        ) || []"
                        :key="player.id"
                        :value="player.id"
                    >
                        {{ usersStore.getPlayerName(player.userId) }}
                    </option>
                </select>
            </label>

            <label class="input input-bordered w-full">
                <span class="label">Amount</span>
                <input
                    v-model.number="tradableSendAmount"
                    type="number"
                    placeholder="Enter value"
                />
            </label>

            <div class="gap-2">
                <button class="btn btn-primary" type="submit">Send</button>
                <button class="btn btn-ghost" type="button" @click="emit('close')">Cancel</button>
            </div>
        </form>
    </div>
</template>

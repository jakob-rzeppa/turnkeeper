<script setup lang="ts">
import { useMessagesEmitter } from '@/emitters/messagesEmitter';
import { useMessagesStore } from '@/stores/messagesStore';
import { usePlayerStore } from '@/stores/playerStore';
import { computed, ref, watch } from 'vue';

const playerStore = usePlayerStore();
const messagesStore = useMessagesStore();

const messagesEmitter = useMessagesEmitter();

const messageInputs = ref<{ [key: number]: string }>({});
const selectedPlayerId = ref<number | null>(null);

// Select first player by default
watch(
    () => playerStore.players,
    (players) => {
        if (players.length > 0 && selectedPlayerId.value === null) {
            selectedPlayerId.value = players[0].id;
        }
    },
    { immediate: true }
);

const selectedPlayer = computed(() => {
    return playerStore.players.find((p) => p.id === selectedPlayerId.value);
});

// By using reversed messages and flex-col-reverse, we can keep the scroll at the bottom
const reversedMessages = computed(() => {
    if (!selectedPlayerId.value || !messagesStore.messages[selectedPlayerId.value]) {
        return [];
    }
    return [...messagesStore.messages[selectedPlayerId.value]].reverse();
});

const sendMessage = () => {
    if (!selectedPlayerId.value) return;
    const content = messageInputs.value[selectedPlayerId.value] || '';
    if (content.trim() === '') return;

    messagesEmitter.sendMessage(selectedPlayerId.value, content);
    messageInputs.value[selectedPlayerId.value] = '';
};

const handleKeyPress = (event: KeyboardEvent) => {
    if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        sendMessage();
    }
};

const selectPlayer = (playerId: number) => {
    selectedPlayerId.value = playerId;
};
</script>

<template>
    <div class="flex flex-col h-[80vh]">
        <h2 class="text-2xl font-bold mb-4">Messages</h2>
        <div class="flex-1 flex flex-row gap-4 min-h-0">
            <!-- Player selection list -->
            <div class="w-64 bg-base-200 rounded-lg p-2 overflow-y-auto">
                <div
                    v-for="player in playerStore.players"
                    :key="player.id"
                    @click="selectPlayer(player.id)"
                    :class="{
                        'p-3 rounded-lg cursor-pointer mb-2 transition-colors': true,
                        'bg-primary text-primary-content': selectedPlayerId === player.id,
                        'hover:bg-base-300': selectedPlayerId !== player.id,
                    }"
                >
                    <div class="font-semibold">{{ player.name }}</div>
                    <div v-if="messagesStore.messages[player.id]?.length" class="text-xs opacity-70">
                        {{ messagesStore.messages[player.id].length }} message{{
                            messagesStore.messages[player.id].length !== 1 ? 's' : ''
                        }}
                    </div>
                </div>
            </div>

            <!-- Chat view -->
            <div class="flex-1 flex flex-col bg-base-200 rounded-lg min-w-0">
                <div v-if="selectedPlayer" class="flex flex-col h-full">
                    <!-- Chat header -->
                    <div class="p-4 border-b border-base-300">
                        <h3 class="text-lg font-bold">{{ selectedPlayer.name }}</h3>
                    </div>

                    <!-- Messages list -->
                    <div
                        v-if="reversedMessages.length > 0"
                        class="flex-1 overflow-y-auto p-4 space-y-2 flex flex-col-reverse"
                    >
                        <div
                            v-for="message in reversedMessages"
                            :key="message.id"
                            :class="{
                                chat: true,
                                'chat-end': message.sendBy === 'gm' || message.sendBy === 'system',
                                'chat-start': message.sendBy === 'player',
                            }"
                        >
                            <div
                                :class="{
                                    'chat-bubble': true,
                                    'chat-bubble-primary':
                                        message.sendBy === 'gm' || message.sendBy === 'system',
                                    'chat-bubble-secondary': message.sendBy === 'player',
                                }"
                            >
                                <div class="text-sm">{{ message.content }}</div>
                                <div class="text-xs opacity-60 mt-1">
                                    {{ new Date(message.timestamp).toLocaleTimeString() }}
                                </div>
                            </div>
                        </div>
                    </div>
                    <div v-else class="flex-1 flex items-center justify-center">
                        <div class="text-sm text-base-content/50 italic">No messages yet.</div>
                    </div>

                    <!-- Input area -->
                    <div class="p-4 border-t border-base-300">
                        <div class="flex gap-2">
                            <input
                                type="text"
                                v-model="messageInputs[selectedPlayerId!]"
                                @keypress="handleKeyPress"
                                placeholder="Type a message..."
                                class="input input-bordered flex-1"
                            />
                            <button
                                class="btn btn-primary"
                                @click="sendMessage"
                                :disabled="!messageInputs[selectedPlayerId!]?.trim()"
                            >
                                Send
                            </button>
                        </div>
                    </div>
                </div>
                <div v-else class="flex items-center justify-center h-full">
                    <div class="text-base-content/50 italic">Select a player to view messages</div>
                </div>
            </div>
        </div>
    </div>
</template>

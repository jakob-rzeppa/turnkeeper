<script setup lang="ts">
import { useMessagesEmitter } from '@/emitters/messagesEmitter';
import { useMessagesStore } from '@/stores/messagesStore';
import { usePlayerStore } from '@/stores/playerStore';
import { computed, ref } from 'vue';

const playerStore = usePlayerStore();
const messagesStore = useMessagesStore();

const messagesEmitter = useMessagesEmitter();

const messageInputs = ref<{ [key: number]: string }>({});

// By using reversed messages and flex-col-reverse, we can keep the scroll at the bottom
const reversedMessages = computed(() => {
    const result: { [key: number]: (typeof messagesStore.messages)[number] } = {};
    for (const playerId in messagesStore.messages) {
        result[playerId] = [...messagesStore.messages[playerId]].reverse();
    }
    return result;
});

const sendMessage = (playerId: number) => {
    const content = messageInputs.value[playerId] || '';
    if (content.trim() === '') return;

    messagesEmitter.sendMessage(playerId, content);
    messageInputs.value[playerId] = '';
};

const handleKeyPress = (event: KeyboardEvent, playerId: number) => {
    if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        sendMessage(playerId);
    }
};
</script>

<template>
    <div class="flex flex-col h-full max-h-[80vh]">
        <h2 class="text-2xl font-bold mb-4">Messages</h2>
        <div class="flex-1 flex flex-row gap-4 flex-wrap">
            <!-- Message cards -->
            <div
                v-for="player in playerStore.players"
                :key="player.id"
                class="card bg-base-200 shadow-md w-[calc(25%-16px)]"
            >
                <div class="card-body p-4">
                    <h3 class="card-title text-lg mb-3">{{ player.name }}</h3>

                    <!-- Messages list -->
                    <div
                        v-if="reversedMessages[player.id] && reversedMessages[player.id].length > 0"
                        class="space-y-2 mb-4 h-64 overflow-y-auto p-2 bg-base-300 rounded-lg flex flex-col-reverse"
                    >
                        <div
                            v-for="message in reversedMessages[player.id]"
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
                    <div v-else class="text-sm text-base-content/50 italic p-4 text-center">
                        No messages yet.
                    </div>

                    <!-- Input area -->
                    <div class="flex gap-2">
                        <input
                            type="text"
                            v-model="messageInputs[player.id]"
                            @keypress="(e) => handleKeyPress(e, player.id)"
                            placeholder="Type a message..."
                            class="input input-bordered input-sm flex-1"
                        />
                        <button
                            class="btn btn-sm btn-primary"
                            @click="sendMessage(player.id)"
                            :disabled="!messageInputs[player.id]?.trim()"
                        >
                            Send
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

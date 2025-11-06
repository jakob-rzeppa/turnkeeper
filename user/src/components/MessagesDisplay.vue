<script setup lang="ts">
import { useMessagesEmitter } from '@/emitters/messagesEmitter';
import { useMessagesStore } from '@/stores/messagesStore';
import { computed, ref } from 'vue';

const messagesStore = useMessagesStore();

const messagesEmitter = useMessagesEmitter();

const reversedMessages = computed(() => {
    return [...messagesStore.messages].reverse();
});

const messageInput = ref<string>('');

const sendMessage = () => {
    const content = messageInput.value || '';
    if (content.trim() === '') return;

    messagesEmitter.sendMessage(content);
    messageInput.value = '';
};

const handleKeyPress = (event: KeyboardEvent) => {
    if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        sendMessage();
    }
};
</script>

<template>
    <div class="divider">Messages</div>
    <div>
        <!-- Messages list -->
        <div
            v-if="reversedMessages && reversedMessages.length > 0"
            class="space-y-2 mb-4 h-64 overflow-y-auto p-2 bg-base-300 rounded-lg flex flex-col-reverse"
        >
            <div
                v-for="message in reversedMessages"
                :key="message.id"
                :class="{
                    chat: true,
                    'chat-start': message.sendBy === 'gm' || message.sendBy === 'system',
                    'chat-end': message.sendBy === 'player',
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
                v-model="messageInput"
                @keypress="(e) => handleKeyPress(e)"
                placeholder="Type a message..."
                class="input input-bordered input-sm flex-3"
            />
            <button
                class="btn btn-sm btn-primary flex-1"
                @click="sendMessage"
                :disabled="!messageInput.trim()"
            >
                Send
            </button>
        </div>
    </div>
</template>

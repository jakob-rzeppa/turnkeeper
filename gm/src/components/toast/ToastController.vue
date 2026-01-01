<script setup lang="ts">
import { useToastStore } from '@/stores/toastStore';
import Toast from './Toast.vue';

/*
 * This component is responsible for rendering the toasts from the toast store.
 */

const toastStore = useToastStore();

const removeToast = (id: number) => {
    toastStore.removeToast(id);
};
</script>

<template>
    <Teleport to="body">
        <TransitionGroup name="toast" tag="ul" class="fixed top-4 right-4 z-50 flex flex-col gap-1">
            <li
                v-for="toast in toastStore.toasts.entries()"
                :key="toast[0]"
                @click="removeToast(toast[0])"
            >
                <Toast
                    :message="toast[1].message + ' (ID: ' + toast[0] + ')'"
                    :type="toast[1].type"
                    :duration="toast[1].duration"
                />
            </li>
        </TransitionGroup>
    </Teleport>
</template>

<style scoped>
.toast-move {
    transition: transform 0.3s ease;
}

.toast-enter-active,
.toast-leave-active {
    transition: all 0.3s ease;
}

.toast-enter-from {
    opacity: 0;
    transform: translateX(100%);
}

.toast-leave-to {
    opacity: 0;
    transform: translateX(100%) scale(0.8);
}

.toast-leave-active {
    position: absolute;
    right: 0;
}
</style>

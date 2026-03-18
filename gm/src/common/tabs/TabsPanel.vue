<script setup lang="ts">
import { onBeforeUnmount, onMounted, shallowRef, type Component } from 'vue';

type TabView = {
    /** Display name shown on the tab. */
    name: string;
    /** Content component rendered when the tab is active. */
    component: Component;
};

const props = defineProps<{
    /** Classes applied to the tabs panel container. */
    class?: string;
    views: TabView[];
}>();

const activeView = shallowRef<TabView | null>(props.views[0] ?? null);

const moveActiveView = (direction: 'prev' | 'next') => {
    if (props.views.length === 0) {
        return;
    }

    const currentIndex = props.views.findIndex(view => view.name === activeView.value?.name);
    const safeIndex = currentIndex >= 0 ? currentIndex : 0;
    const offset = direction === 'next' ? 1 : -1;
    const nextIndex = (safeIndex + offset + props.views.length) % props.views.length;

    activeView.value = props.views[nextIndex] ?? null;
};

const onKeydown = (event: KeyboardEvent) => {
    if (!event.metaKey) {
        return;
    }

    if (event.key === 'ArrowRight') {
        event.preventDefault();
        moveActiveView('next');
        return;
    }

    if (event.key === 'ArrowLeft') {
        event.preventDefault();
        moveActiveView('prev');
    }
};

onMounted(() => {
    // A global keydown listener is used to allow switching tabs even when the tab content is not focused.
    window.addEventListener('keydown', onKeydown);
});

onBeforeUnmount(() => {
    window.removeEventListener('keydown', onKeydown);
});
</script>

<template>
    <div :class="['w-full h-screen', props.class]">
        <div role="tablist" class="tabs tabs-lift w-full h-full bg-base-300">
            <template v-for="view in props.views" :key="view.name">
                <div
                    role="tab"
                    class="tab"
                    :class="{ 'tab-active': activeView?.name === view.name }"
                    @click="activeView = view"
                >
                    {{ view.name }}
                </div>
                <div
                    class="tab-content bg-base-100 p-4 border-0 border-t border-base-300 rounded-none overflow-scroll"
                >
                    <component :is="view.component" />
                </div>
            </template>
        </div>
    </div>
</template>

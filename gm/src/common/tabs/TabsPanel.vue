<script setup lang="ts">
import { shallowRef, type Component } from 'vue';

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
</script>

<template>
    <div :class="['w-full h-full', props.class]">
        <div role="tablist" class="tabs tabs-lift w-full bg-base-300">
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
                    class="tab-content bg-base-100 p-4 border-0 border-t border-base-300 rounded-none"
                >
                    <component :is="view.component" />
                </div>
            </template>
        </div>
    </div>
</template>

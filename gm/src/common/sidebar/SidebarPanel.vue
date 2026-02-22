<script setup lang="ts">
import { computed, markRaw, ref, shallowRef, useTemplateRef, type Component } from 'vue';

/** A single entry in the sidebar activity bar. */
interface SidebarView {
    /** Icon component rendered in the activity bar button. */
    icon: Component;
    /** Display name shown as the tooltip and panel heading. */
    name: string;
    /** Content component rendered inside the side panel. */
    component: Component;
}

const props = defineProps<{
    /** Ordered list of views to populate the activity bar. */
    views: SidebarView[];
}>();

/** Maximum width of the side panel in pixels. */
const MAX_WIDTH = 600;
/** Minimum width of the side panel in pixels before it auto-hides. */
const MIN_WIDTH = 150;

const DEFAULT_WIDTH = 240;

const sidePanelEl = useTemplateRef('sidePanel');
const activeView = shallowRef<SidebarView | null>(props.views[0] ?? null);
const isHidden = ref(false);
const panelWidth = ref(DEFAULT_WIDTH);
const isResizing = ref(false);

/**
 * Pre-process views so icons and components are marked as raw, preventing Vue
 * from making them reactive and improving render performance.
 */
const processedViews = computed(() =>
    props.views.map(v => ({
        ...v,
        icon: markRaw(v.icon),
        component: markRaw(v.component),
    }))
);

/**
 * Activate the given view. If it is already active, toggle the panel's
 * visibility instead of switching content.
 */
const setActiveView = (view: SidebarView) => {
    if (isHidden.value) {
        // Reset to default width when re-showing the panel after it was hidden
        panelWidth.value = DEFAULT_WIDTH;
    }

    if (activeView.value?.name === view.name) {
        isHidden.value = !isHidden.value;
        return;
    }

    activeView.value = view;
    isHidden.value = false;
};

/**
 * Begin a drag-resize interaction on the side panel.
 * Clamps the panel width between MIN_WIDTH and MAX_WIDTH, and auto-hides the
 * panel when the user drags past the left edge.
 */
const startResize = () => {
    isResizing.value = true;

    const onMouseMove = (e: MouseEvent) => {
        const panelLeft = sidePanelEl.value!.getBoundingClientRect().left;
        const desiredWidth = e.clientX - panelLeft;

        panelWidth.value = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, desiredWidth));

        if (isHidden.value && desiredWidth > MIN_WIDTH) {
            isHidden.value = false;
        } else if (!isHidden.value && desiredWidth < 0) {
            isHidden.value = true;
        }
    };

    const onMouseUp = () => {
        isResizing.value = false;
        window.removeEventListener('mousemove', onMouseMove);
        window.removeEventListener('mouseup', onMouseUp);
    };

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
};
</script>

<template>
    <div class="flex flex-row h-screen">
        <!-- Activity Bar: one button per registered view -->
        <div class="flex flex-col items-center bg-base-300 w-14 shrink-0">
            <button
                v-for="view in processedViews"
                :key="view.name"
                class="tooltip tooltip-right w-full items-center aspect-square relative hover:bg-base-content/10 transition-colors"
                :class="
                    activeView?.name === view.name ? 'text-base-content' : 'text-base-content/40'
                "
                :data-tip="view.name"
                @click="setActiveView(view)"
            >
                <!-- Active indicator stripe -->
                <div
                    v-if="activeView?.name === view.name"
                    class="absolute left-0 top-0 bottom-0 w-0.5 bg-primary"
                />
                <component :is="view.icon" />
            </button>
        </div>

        <!-- Side Panel: displays the active view's content -->
        <div
            ref="sidePanel"
            v-show="!isHidden"
            class="bg-base-200 p-4 overflow-hidden"
            :style="{ width: `${panelWidth}px` }"
        >
            <div
                class="mb-2 text-xs font-bold uppercase tracking-widest text-base-content/60 select-none"
            >
                {{ activeView?.name }}
            </div>
            <div class="overflow-y-auto flex-1">
                <component :is="activeView?.component" />
            </div>
        </div>

        <!-- Resize Handle: draggable divider between the panel and main content -->
        <div
            class="w-1 cursor-col-resize transition-colors shrink-0"
            :class="isResizing ? 'bg-primary/50' : 'bg-transparent hover:bg-primary/50'"
            @mousedown="startResize"
        />
    </div>
</template>

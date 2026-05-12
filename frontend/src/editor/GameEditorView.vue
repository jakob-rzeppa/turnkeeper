<script setup lang="ts">
import { onMounted } from 'vue';
import { useRoute } from 'vue-router';
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import OverviewIcon from './OverviewIcon.vue';
import OverviewSidebar from './OverviewSidebar.vue';
import { useGameEditor } from './useGameEditor';
import CodeEditor from './CodeEditor.vue';
import TabsPanel from '../common/tabs/TabsPanel.vue';
import CodeCheck from './CodeCheck.vue';

const route = useRoute();

const gameEditor = useGameEditor();

onMounted(() => {
    gameEditor.loadGame(route.params.id as string);
});
</script>

<template>
    <div class="flex w-screen h-screen">
        <SidebarPanel
            :views="[{ icon: OverviewIcon, name: 'Overview', component: OverviewSidebar }]"
        />
        <TabsPanel
            :views="[
                { name: 'Editor', component: CodeEditor },
                { name: 'Check', component: CodeCheck },
            ]"
        />
    </div>
</template>

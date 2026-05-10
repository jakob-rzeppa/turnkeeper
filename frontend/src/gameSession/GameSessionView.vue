<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useSession } from './useSession';
import { useRoute } from 'vue-router';
import TabsPanel from '../common/tabs/TabsPanel.vue';
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import ConnectionSidebar from './sidebar/ConnectionSidebar.vue';
import ConnectionSidebarIcon from './sidebar/ConnectionSidebarIcon.vue';
import OverviewTab from './tabs/OverviewTab.vue';
import PlayerOrganisationTab from './tabs/PlayerOrganisationTab.vue';

const route = useRoute();

const session = useSession();

const connectToSession = () => {
    session.connect(route.params.gameId as string, route.params.gameInstanceId as string);
};

const disconnectFromSession = () => {
    if (session.connectionStatus.value === 'connected') {
        session.disconnect();
    }
};

onMounted(() => {
    connectToSession();
});

onUnmounted(() => {
    disconnectFromSession();
});
</script>

<template>
    <div>
        <div class="flex w-screen h-screen">
            <SidebarPanel
                :views="[
                    {
                        icon: ConnectionSidebarIcon,
                        name: 'Connection',
                        component: ConnectionSidebar,
                    },
                ]"
            />
            <TabsPanel
                :views="[
                    { name: 'Overview', component: OverviewTab },
                    { name: 'Player Organisation', component: PlayerOrganisationTab },
                ]"
            />
            <SidebarPanel class="ml-auto" :views="[]" side="right" />
        </div>
    </div>
</template>

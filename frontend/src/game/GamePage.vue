<script setup lang="ts">
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import TabsPanel from '../common/tabs/TabsPanel.vue';
import DisconnectSidebar from './disconnect/DisconnectSidebar.vue';
import DisconnectIcon from './disconnect/DisconnectIcon.vue';
</script>

<template>
    <div class="flex w-screen h-screen">
        <SidebarPanel
            :views="[
                {
                    icon: DisconnectIcon,
                    name: 'Disconnect',
                    component: DisconnectSidebar,
                },
            ]"
        />
        <SidebarPanel class="ml-auto" :views="[]" side="right" />
    </div>
</template>

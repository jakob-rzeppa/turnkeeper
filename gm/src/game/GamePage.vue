<script setup lang="ts">
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import TabsPanel from '../common/tabs/TabsPanel.vue';
import RoundOverview from './round/RoundOverview.vue';
import NotesEditorSidebar from './notes/NotesEditorSidebar.vue';
import NotesIcon from './notes/NotesIcon.vue';
import TradablesView from './tradables/TradablesView.vue';
import PlayerOrganisationView from './playerOrganisation/PlayerOrganisationView.vue';
import DisconnectSidebar from './disconnect/DisconnectSidebar.vue';
import DisconnectIcon from './disconnect/DisconnectIcon.vue';
</script>

<template>
    <div class="flex w-screen h-screen">
        <SidebarPanel
            :views="[
                { icon: NotesIcon, name: 'Notes', component: NotesEditorSidebar },
                {
                    icon: DisconnectIcon,
                    name: 'Disconnect',
                    component: DisconnectSidebar,
                },
            ]"
        />
        <TabsPanel
            :views="[
                { name: 'Round Overview', component: RoundOverview },
                { name: 'Tradables', component: TradablesView },
                { name: 'Player Organisation', component: PlayerOrganisationView },
            ]"
        />
        <SidebarPanel class="ml-auto" :views="[]" side="right" />
    </div>
</template>

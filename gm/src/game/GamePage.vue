<script setup lang="ts">
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import TestView1 from './TestView1.vue';
import TestView2 from './TestView2.vue';
import TestIcon1 from './TestIcon1.vue';
import TestIcon2 from './TestIcon2.vue';
import TabsPanel from '../common/tabs/TabsPanel.vue';
import PlayerOrganisationView from './mainViews/PlayerOrganisationView.vue';
</script>

<template>
    <div class="flex w-screen h-screen">
        <SidebarPanel
            :views="[
                { icon: TestIcon1, name: 'Test View 1', component: TestView1 },
                { icon: TestIcon2, name: 'Test View 2', component: TestView2 },
            ]"
        />
        <TabsPanel :views="[{ name: 'Player Organisation', component: PlayerOrganisationView }]" />
        <SidebarPanel
            class="ml-auto"
            :views="[
                { icon: TestIcon1, name: 'Test View 1', component: TestView1 },
                { icon: TestIcon2, name: 'Test View 2', component: TestView2 },
            ]"
            side="right"
        />
    </div>
</template>

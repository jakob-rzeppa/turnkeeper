<script setup lang="ts">
import { useWsConnection } from '../api/useWsConnection';
import SidebarPanel from '../common/sidebar/SidebarPanel.vue';
import TestView1 from './TestView1.vue';
import TestView2 from './TestView2.vue';
import TestIcon1 from './TestIcon1.vue';
import TestIcon2 from './TestIcon2.vue';

const wsConnection = useWsConnection();
</script>

<template>
    <div class="flex flex-row w-screen">
        <SidebarPanel
            :views="[
                { icon: TestIcon1, name: 'Test View 1', component: TestView1 },
                { icon: TestIcon2, name: 'Test View 2', component: TestView2 },
            ]"
        />
        <div>
            <h1>Game Page</h1>
            <p>This is the game page.</p>
            <button @click="wsConnection.send('Hi')" class="btn btn-primary">Send Message</button>
            <button @click="wsConnection.disconnect" class="btn btn-outline btn-secondary">
                Disconnect
            </button>
        </div>
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

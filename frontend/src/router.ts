import { createRouter, createWebHistory } from 'vue-router';
import HomeView from './views/HomeView.vue';
import GamesView from './views/GamesView.vue';

const routes = [
    { path: '/', name: 'home', component: () => HomeView },
    { path: '/games', name: 'games', component: () => GamesView },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

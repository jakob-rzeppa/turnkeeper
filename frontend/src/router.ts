import { createRouter, createWebHistory } from 'vue-router';
import HomeView from './views/HomeView.vue';
import GameOverviewView from './views/GameOverviewView.vue';

const routes = [
    { path: '/', name: 'home', component: () => HomeView },
    { path: '/games', name: 'games', component: () => GameOverviewView },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

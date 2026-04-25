import { createRouter, createWebHistory } from 'vue-router';
import HomeView from './views/HomeView.vue';
import GameOverviewView from './views/GameOverviewView.vue';
import GameDetailsView from './views/GameDetailsView.vue';

const routes = [
    { path: '/', name: 'home', component: () => HomeView },
    { path: '/games', name: 'gameOverview', component: () => GameOverviewView },
    { path: '/games/:id', name: 'game', component: () => GameDetailsView },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

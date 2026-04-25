import { createRouter, createWebHistory } from 'vue-router';
import HomeView from './views/HomeView.vue';
import GameOverviewView from './views/GameOverviewView.vue';

const routes = [
    { path: '/', name: 'home', component: () => HomeView },
    { path: '/games', name: 'game-overview', component: () => GameOverviewView },
    { path: '/games/:id', name: 'game', component: () => import('./views/GameDetailsView.vue') },
    {
        path: '/games/:id/editor',
        name: 'game-editor',
        component: () => import('./views/GameEditorView.vue'),
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

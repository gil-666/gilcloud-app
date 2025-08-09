import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/login',
            name: 'login',
            component: () => import('../components/Login.vue'),
        },
        {
            path: '/',
            name: 'home',
            component: () => import('../views/FileManager.vue'),
        },
        {
            path: '/vm',
            name: 'vm',
            component: () => import('../views/VirtualMachines.vue'),
        },
    ],
})

export default router
// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: () => import('../views/FileManager.vue'),
    meta: { title: 'GilCloud | Drive' },
  },
  {
    path: '/movies',
    name: 'movies',
    component: () => import('../views/Movies.vue'),
    meta: { title: 'GilCloud | Movies' },
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('../components/Login.vue'),
    meta: { title: 'GilCloud | Login' },
  },
]

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

// Optional: dynamic title based on route meta
router.beforeEach((to, from, next) => {
  document.title = to.meta.title as string || 'GilCloud'
  next()
})
export default router
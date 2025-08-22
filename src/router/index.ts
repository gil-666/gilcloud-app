// link/router/index.ts
declare module 'vue-router' {
  interface RouteMeta {
    title?: string
    description?: string
    public?: boolean
  }
}
import { createRouter, createWebHistory, createMemoryHistory, RouteRecordRaw } from 'vue-router'
import { useApiUrl } from '../main';
const isServer = typeof window === 'undefined';
const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: () => import('../views/FileManager.vue'),
    meta: { title: 'GilCloud | Drive' },
  },
  {
    path: '/view',
    name: 'view',
    children: [
      {
        path: 'photo',
        name: 'photo',
        component: () => import('../components/media/ImageViewer.vue'),
        props: route => ({
          link: route.query.link ? `${useApiUrl()}/download/${route.query.link}` : null
        }),
        meta: { title: 'GilCloud | Image', public: true },
      },
      {
        path: 'video',
        name: 'video',
        component: () => import('../components/media/VideoPlayer.vue'),
        props: route => ({
          link: route.query.link ? `${useApiUrl()}/download/${route.query.link}` : null
        }),
        meta: { title: 'GilCloud | Video', public: true },
      },
      {
        path: 'audio',
        name: 'audio',
        component: () => import('../components/media/AudioPlayer.vue'),
        props: route => ({
          link: route.query.link ? `${useApiUrl()}/download/${route.query.link}` : null
        }),
        meta: { title: 'GilCloud | Audio', public: true },
      }]
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
  history: isServer ? createMemoryHistory() : createWebHistory(),
  routes,
})

// Optional: dynamic title based on route meta

export default router
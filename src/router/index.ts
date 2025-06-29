import { createRouter, createWebHistory } from 'vue-router'
import FileManager from "../views/FileManager.vue";
import {useAppStore} from "../stores/app.ts";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: FileManager,
            props: () => {
                const store = useAppStore();
                return { dir: store.userHomeDir };
            }
        },
    ],
})

export default router
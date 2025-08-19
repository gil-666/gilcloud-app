import { ViteSSG } from 'vite-ssg';
import { createPinia } from 'pinia';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';
import { router } from './router';

export const createApp = ViteSSG(
  App,
  { routes: router.options.routes },
  ({ app, router, isClient, initialState }) => {
    // Plugins
    app.use(PrimeVue, { unstyled: true });
    app.use(createPinia());
    app.use(ToastService);

    // Global API URL accessible in components via this.$apiUrl
    const apiUrl = import.meta.env.VITE_API_URL;
    // app.config.globalProperties.$apiUrl = apiUrl;

    // Only set useApiUrl() on client
  }
);
export function useApiUrl(){
  return import.meta.env.VITE_API_URL
}
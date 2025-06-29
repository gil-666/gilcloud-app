import { createApp } from "vue";
import {createPinia} from "pinia";
import App from "./App.vue";
import PrimeVue from 'primevue/config'
import router from './router'
window.API_URL = import.meta.env.VITE_API_URL;
createApp(App).use(PrimeVue, {
    unstyled: true
}).use(router).use(createPinia()).mount("#app");

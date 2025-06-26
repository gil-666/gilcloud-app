import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config'
import router from './router'

createApp(App).use(PrimeVue, {
    unstyled: true
}).use(router).mount("#app");

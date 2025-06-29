// src/stores/app.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAppStore = defineStore('app', () => {
    const userHomeDir = ref(''); // example of global state
    const language = ref('en');
    const currentDir = ref('')

    function setUserHomeDir(dir: string) {
        userHomeDir.value = dir;
    }

    function setCurrentDir(dir: string) { //sets the current directory in view
        currentDir.value = dir;
    }

    function setLanguage(lang: string) {
        language.value = lang;
    }

    return {
        userHomeDir,
        language,
        currentDir,
        setCurrentDir,
        setUserHomeDir,
        setLanguage
    };
});

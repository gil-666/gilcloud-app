// src/stores/app.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAppStore = defineStore('app', () => {
    const userHomeDir = ref(''); // example of global state
    const language = ref('en');
    const currentDir = ref('')
    const progress = ref('')
    const username = ref(localStorage.getItem('username') || '');
    type StorageCount = {
        maxStorage: number;
        currentUsage: number;
    };

    const storageCount = ref<StorageCount>({
        maxStorage: 0,
        currentUsage: 0,
    });
    function setUserHomeDir(dir: string) {
        userHomeDir.value = dir;
        setCurrentDir(dir)
    }

    function calculateProgress(){
        progress.value = ((storageCount.value.currentUsage / storageCount.value.maxStorage) * 100).toFixed(2)
    }
    function setStorageCount(stor:StorageCount) {
        storageCount.value = stor;
        calculateProgress()
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
        username,
        storageCount,
        progress,
        calculateProgress,
        setStorageCount,
        setCurrentDir,
        setUserHomeDir,
        setLanguage
    };
});

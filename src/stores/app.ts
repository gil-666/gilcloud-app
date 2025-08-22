// src/stores/app.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { getLocalStorageItem } from '../util/browser';

export const useAppStore = defineStore('app', () => {
    const userHomeDir = ref(''); // example of global state
    const language = ref('en');
    const currentDir = ref('')
    const progress = ref('')
    const isLoggedIn = ref(false);
    const username = ref(getLocalStorageItem('username', ''))

    const UIEvents = ref({
        showFileDropdown: false,
        showMenuBar: false
    });
    type StorageCount = {
        maxStorage: number;
        currentUsage: number;
    };

    const storageCount = ref<StorageCount>({
        maxStorage: 0,
        currentUsage: 0,
    });
    function setIsLoggedIn(value:boolean){
        isLoggedIn.value = value;
    }
    function closeMenus(){
        UIEvents.value.showMenuBar = false;
        UIEvents.value.showFileDropdown = false;
    }
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
        UIEvents,
        progress,
        closeMenus,
        calculateProgress,
        setStorageCount,
        setIsLoggedIn,
        setCurrentDir,
        setUserHomeDir,
        setLanguage
    };
});

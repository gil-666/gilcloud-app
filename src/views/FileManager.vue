<template>
      <NewFolder
          @close="newFolderVisible = false"
          @submit="createFolder"
          v-if="newFolderVisible"
      ></NewFolder>


  <main
      @click="store.closeMenus()"
      class="bg-neutral-800 w-full lg:p-5 p-2 pt-2 text-center h-full overflow-hidden"
  >
    <div class="flex relative place-items-center w-full justify-center">
      <div class="controls lg:absolute mr-5 left-5">
        <i
            v-if="navHistory.length > 0"
            class="pi pi-arrow-left cursor-pointer"
            @click="goBackDir"
            style="font-size: 18pt"
            title="Go back"
        ></i>
        <i
            v-if="navHistory.length > 0"
            class="pi pi-home cursor-pointer lg:ml-4 ml-2"
            @click="resetDir"
            style="font-size: 20pt"
            title="Go to root directory"
        ></i>
        <i
            @click="newFolderVisible = true"
            class="pi pi-plus-circle cursor-pointer lg:ml-4 ml-2"
            style="font-size: 20pt"
            title="New Folder"
        ></i>
      </div>

      <h1
          class="text-3xl pb-2 relative text-center"
          :title="formatDirText(currentDir,true)"
      >{{ formatDirText(currentDir) }}</h1>
    </div>

    <div class="border-1 border-neutral-600 m-5 relative bottom-5 p-10 overflow-y-auto h-full max-h-11/12">
          <DirectoryContent v-model:dir="currentDir" :key="currentDir" />


    </div>
  </main>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useAppStore } from "../stores/app.ts";
import NewFolder from "@/components/filemanager/NewFolder.vue";
import DirectoryContent from "@/components/filemanager/DirectoryContent.vue";
import Loader from "@/components/Loader.vue";

const store = useAppStore();
const newFolderVisible = ref(false);
const currentDir = computed({
  get() {
    return store.currentDir.replace(/^"|"$/g, "");
  },
  set(value: string) {
    const previous = store.currentDir.replace(/^"|"$/g, "");
    if (previous !== value) {
      navHistory.push(previous);
      store.setCurrentDir(value);
    }
  },
});
let navHistory: Array<string> = [];

async function goBackDir() {
  const prev = navHistory.pop();
  if (prev) {
    currentDir.value = prev;
  }
}

async function resetDir() {
  currentDir.value = "./" + store.userHomeDir.replace(/^"|"$/g, "");
  navHistory = [];
}

async function reloadDir(){
  currentDir.value = "./" + store.currentDir.replace(/^"|"$/g, "");
  navHistory = [];
}

function formatDirText(text: string, getUncut = false) {
  const marker = `data/storage/user/`;
  const normalized = text.replaceAll("\\", "/"); // normalize slashes
  const index = normalized.indexOf(marker);
  const uncut = text.slice(index + marker.length).replaceAll("\\", "/")
  if(getUncut){
    return uncut
  }
  if(uncut.length > 22){
    return "..."+String(uncut).slice(uncut.length-22,uncut.length);
  }else{
    return uncut
  }

}

// createFolder delegates to directory content via prop submission; keep same logic here for closing
async function createFolder(foldername: string) {
  // The DirectoryContent will react to currentDir change automatically via v-model
  // This is just for API call; mimic original behavior so NewFolder works at root of currentDir
  await fetch(`${window.API_URL}/create_folder`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      parent_dir: `${currentDir.value}`,
      folder_name: `${foldername}`,
    }),
  });
  newFolderVisible.value = false;
  await reloadDir()
  // Let DirectoryContent reactively reload based on currentDir (it already does in its async setup)
}
</script>

<style scoped>
/* keep scoped clean */
</style>

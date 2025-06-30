<script setup lang="ts">
import Folder from "../components/filemanager/Folder.vue";
import Button from "../volt/Button.vue"
import File from "../components/filemanager/File.vue";
import {computed, ref, watch} from "vue";
import { useAppStore } from "../stores/app.ts";
import axios from "axios";
import {useToast} from "primevue";
import NewFolder from "@/components/filemanager/NewFolder.vue";

const store = useAppStore();
const toast = useToast();
const newFolderVisible = ref(false);
const currentDir = computed({
  get() {
    return store.currentDir.replace(/^"|"$/g, '');
  },
  set(value: string) {
    store.setCurrentDir(value);
  }
});
const folders = ref([]);
const files = ref([]);

let navHistory: Array<string> = [];

// Main directory loader
async function loadDirectory() {
  folders.value = await getFolders();
  files.value = await getFiles();
}
async function getFolders() {
  const response = await axios.get(`${window.API_URL}/folders`, { params: { dir: currentDir.value } });
  return response.data;
}

async function getFiles() {
  const response = await axios.get(`${window.API_URL}/files`, { params: { dir: currentDir.value } });
  return response.data;
}

// navigation
async function changeDir(newDir: string) {
  navHistory.push(currentDir.value);
  currentDir.value = newDir.replaceAll("\\", "/");
  store.setCurrentDir(newDir);
  await loadDirectory();
}

async function goBackDir() {
  const prev = navHistory.pop();
  if (prev) {
    currentDir.value = prev;
  }
}

async function resetDir() {
  navHistory = [];
  currentDir.value = "./" + store.userHomeDir.replace(/^"|"$/g, '');
}
function formatDirText(text: string) {
  const limit = 50;
  return text.length > limit ? "..." + text.slice(-limit) : text;
}

const isDirEmpty = () => {
  return folders.value.length === 0 && files.value.length === 0;
};
async function updateStorageCount() {
  const response = await axios.get(`${window.API_URL}/storage/${localStorage.getItem('username')}`);
  store.setStorageCount(response.data);
}
async function downloadFile(fileName: string) {
  const username = localStorage.getItem("username");
  console.log("Downloading file...");
  window.open(`${window.API_URL}/download/${username}/${fileName}`);
}

async function deleteFile(fileName: string){
  const username = localStorage.getItem("username");
  console.log("Deleting file...");
  try {
    await axios.delete(`${window.API_URL}/delete/${username}/${fileName}`);
    await loadDirectory()
    toast.add({
      detail: `File ${fileName} was deleted successfully`,
      summary: "File deleted",
      severity: "info",
      life: 3000,
    })
    await updateStorageCount()
  } catch (e) {
    console.error("Delete failed:", e);
  }
}

async function createFolder(foldername: string) {
  await axios.post("http://localhost:8080/create_folder", {
    parent_dir: `${currentDir.value}`,
    folder_name: `${foldername}`
  });
  newFolderVisible.value = false;
  await loadDirectory();
  await updateStorageCount();
};

// Load after login
watch(
    currentDir,
    async (newDir) => {
      if (newDir) {
        await loadDirectory();
      }
    },
    { immediate: true }
);
</script>

<template>
  <NewFolder @close="newFolderVisible = false" @submit="createFolder" v-if="newFolderVisible"></NewFolder>
  <main class="bg-neutral-800 w-full lg:p-5 p-2 pt-2 text-center h-full overflow-hidden">
    <div class="flex relative place-items-center w-full justify-center">
      <div class="controls lg:absolute mr-5 left-5">
        <i v-if="navHistory.length >0" class="pi pi-arrow-left cursor-pointer" @click="goBackDir"
           style="font-size: 18pt"></i>
        <i v-if="navHistory.length >0" class="pi pi-home cursor-pointer lg:ml-4 ml-2" @click="resetDir"
           style="font-size: 20pt"></i>
      </div>

      <h1 class="text-3xl pb-2 relative text-center" :title="currentDir">{{ formatDirText(currentDir) }}</h1>
      <div class="controls ml-2 flex place-items-center">
        <i @click="newFolderVisible = true" class="pi pi-plus-circle cursor-pointer lg:ml-4 ml-2"
           style="font-size: 20pt" title="New Folder"></i>
      </div>
    </div>
    <div class="border-1 border-neutral-600 m-5 relative bottom-5 p-10 overflow-y-auto h-full max-h-11/12">
      <p v-if="isDirEmpty()" class="font-light">No items to show</p>

      <div class="file-list gap-10 grid 2xl:grid-cols-8 lg:grid-cols-4 grid-cols-2 justify-items-center  ">
        <Folder class="" v-for="folder in folders" v-bind="folder" @click="changeDir(folder.path)"></Folder>
        <File @click="downloadFile(file.name)" @delete="deleteFile(file.name)" v-for="file in files" v-bind="file"></File>
      </div>

    </div>
  </main>

</template>

<style scoped>

</style>
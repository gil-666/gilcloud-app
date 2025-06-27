<script setup lang="ts">
import Folder from "../components/filemanager/Folder.vue";
import File from "../components/filemanager/File.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const folders = ref();
const files = ref();
const dir = ref("../"); //current directory
folders.value = await getFolders();
files.value = await getFiles();
const isDirEmpty = () => {
  console.log(folders.value.length)
  return folders.value.length <= 0 && files.value.length <= 0;
}

async function getFolders() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // folders.value = folders.value.map((name:String, index:Number) => ({
  //   id: index,
  //   name,
  //   path: `${dir.value.endsWith('/') ? dir.value : dir.value + '/'}${name}`
  // }))
  return await invoke("get_folders", {dir: dir.value})
}

async function getFiles() {
  const fil = await invoke("get_files", {dir: dir.value});
  console.log(files);
  return fil
}

async function changeDir(newDir) {
  dir.value = newDir.replaceAll("\\", "/");
  folders.value = await getFolders();
  files.value = await getFiles();
}


</script>

<template>
  <main class="bg-neutral-800 w-full p-5 pt-2 text-center h-full overflow-hidden">
    <div class="flex relative place-items-center w-full justify-center">
      <i class="pi pi-arrow-left absolute left-5" style="font-size: 18pt"></i>
      <h1 class="text-3xl pb-2 relative text-center">{{ dir }}</h1>
    </div>
    <div class="border-1 border-neutral-600 m-5 relative bottom-5 p-10 overflow-y-auto h-full max-h-11/12">

      <p v-if="isDirEmpty()" class="font-light">No items to show</p>

      <div class="file-list gap-10 grid 2xl:grid-cols-8 lg:grid-cols-4 grid-cols-2 justify-items-center  ">
        <Folder class="" v-for="folder in folders" v-bind="folder" @click="changeDir(folder.path)"></Folder>
        <File v-for="file in files" v-bind="file"></File>
      </div>

    </div>
  </main>

</template>

<style scoped>

</style>
<script setup lang="ts">
import Folder from "../components/filemanager/Folder.vue";
import File from "../components/filemanager/File.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const folders = ref();
const files = ref();
const dir = ref("/home/gil"); //current directory
const homeDir = dir.value

folders.value = await getFolders();
files.value = await getFiles();
const isDirEmpty = () => {
  console.log(folders.value.length)
  return folders.value.length <= 0 && files.value.length <= 0;
}
let navHistory: Array<String> = []
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
  navHistory.push(dir.value) //save current dir before changing
  dir.value = newDir.replaceAll("\\", "/");
  folders.value = await getFolders();
  files.value = await getFiles();
}

async function goBackDir() {
  dir.value = navHistory[navHistory.length - 1].toString()
  navHistory.pop()
  folders.value = await getFolders();
  files.value = await getFiles();
}

async function resetDir(){
  dir.value = homeDir;
  navHistory = []
  folders.value = await getFolders();
  files.value = await getFiles();
}

function formatDirText(text: string){
  let limit = 50
  if(text.length > limit){
    return "..."+text.slice(text.length - limit, text.length)
  }else{
    return text;
  }
}
</script>

<template>
  <main class="bg-neutral-800 w-full p-5 pt-2 text-center h-full overflow-hidden">
    <div class="flex relative place-items-center w-full justify-center">
      <div class="controls lg:absolute mr-5 left-5">
        <i v-if="navHistory.length >0" class="pi pi-arrow-left cursor-pointer" @click="goBackDir"
           style="font-size: 18pt"></i>
        <i v-if="navHistory.length >0" class="pi pi-home cursor-pointer lg:ml-4 ml-2" @click="resetDir"
           style="font-size: 20pt"></i>
      </div>

      <h1 class="text-3xl pb-2 relative text-center" :title="dir">{{ formatDirText(dir) }}</h1>
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
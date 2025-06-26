<script setup lang="ts">
import Folder from "../components/filemanager/Folder.vue";
import File from "../components/filemanager/File.vue";
import {ref} from "vue";

const folders = ref([{
  "name": "bruh"
}, {
  "name": "bruh2"
}, {
  "name": "bruh3"
}, {
  "name": "bruh4"
}, {
  "name": "bruh5"
}, {
  "name": "bruh6"
}, {
  "name": "bruh7"
},{
  "name": "bruh8"
},{
  "name": "bruh9"
}]);

const files = ref([{
  "name": "sexoo duro.png",
  "size": 4194304
}])

</script>

<template>
  <main class="bg-neutral-800 w-full p-5 text-center h-full">
    <div class="border-1 border-neutral-600 m-5 relative bottom-5 p-10 overflow-y-auto h-full">
      <div class="file-list gap-10 grid lg:grid-cols-8 grid-cols-4 justify-items-center  " >
        <Folder class="" v-for="folder in folders" v-bind="folder"></Folder>
        <File v-for="file in files" v-bind="file"></File>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>
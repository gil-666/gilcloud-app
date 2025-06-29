<script setup lang="ts">
import {Image} from "primevue";
import {ref} from "vue";
const imageSources = ref(['/src/assets/m1.jpg', '/src/assets/m2.jpg']);
const random = Math.floor(Math.random() * imageSources.value.length);
</script>

<template>
  <main class="bg-neutral-800 w-full p-5 pt-2 text-center h-full overflow-hidden">
    <div class="block relative place-items-center w-full justify-center">
    <h1>NO WORKY</h1>
      <Image width="350" :src="imageSources[random]"></Image>
    </div>
    </main>
</template>

<style scoped>

</style>
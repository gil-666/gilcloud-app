<script setup lang="ts">
import {Image} from "primevue";
import Button from "../volt/Button.vue";
import {ref} from "vue";
const win = window;
import m1 from '@/assets/m1.jpg';
import m2 from '@/assets/m2.jpg';
import m3 from '@/assets/m3.jpg';
import m4 from '@/assets/m4.jpg';
import m5 from '@/assets/m5.jpg';
import m6 from '@/assets/m6.jpg';
import m7 from '@/assets/m7.jpg';
import m8 from '@/assets/m8.jpg';

const imageSources = ref([
  { id: 1, name: 'Image 1', src: m1, alt: 'Image 1 description' },
  { id: 2, name: 'Image 2', src: m2, alt: 'Image 2 description' },
  { id: 3, name: 'Image 3', src: m3, alt: 'Image 3 description' },
  { id: 4, name: 'Image 4', src: m4, alt: 'Image 4 description' },
  { id: 5, name: 'Image 5', src: m5, alt: 'Image 5 description' },
  { id: 6, name: 'Image 6', src: m6, alt: 'Image 6 description' },
  { id: 7, name: 'Image 7', src: m7, alt: 'Image 7 description' },
  { id: 8, name: 'Image 8', src: m8, alt: 'Image 8 description' },
]);

const random = Math.floor(Math.random() * imageSources.value.length);
</script>

<template>
  <main class="bg-neutral-800 w-full p-5 pt-2 text-center h-full overflow-hidden">
    <div class="block relative place-items-center w-full justify-center">
    <h1 class="text-3xl">NO WORKY</h1>
      <Image
          :src="imageSources[random].src"
          image-style="width: auto; max-height: 430px; object-fit: contain"
      />
      <Button class="mt-2" style="background-color: #0d542b;" @click="win.location.reload()">GOOD LUCK</Button>
    </div>
    </main>
</template>

<style scoped>
</style>
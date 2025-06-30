<script setup lang="ts">
import Button from "@/volt/Button.vue";
import {defineEmits, ref} from "vue";
const emit = defineEmits<{
  (event: 'submit', foldername: string): void,
  (event: 'close'): void;
}>();
const foldername = ref('')
</script>

<template>
  <div class="flex backdrop-blur-xl fixed top-0 h-full w-full z-70">

    <div class="flex items-center justify-center w-full h-full">
      <div class="items-center justify-center w-70">
        <i @click="emit('close')" class="pi pi-times relative left-62" style="font-size: 20px"></i>
        <div class="bg-neutral-900 p-5 rounded-2xl shadow-2xl text-center">
          <p>Create a new folder</p>
          <input v-model="foldername" class="folder-name border-1 border-neutral-400 rounded-sm mt-2" type="text">
          <Button @click="emit('submit',foldername)" class="mt-2" label="OK"></Button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.folder-name{
  cursor: text;
  background-color: #262626;
}
</style>
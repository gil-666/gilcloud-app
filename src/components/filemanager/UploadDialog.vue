<script setup lang="ts">
import {defineEmits, onMounted, onUnmounted, ref} from 'vue';
import {getCurrentWebview} from '@tauri-apps/api/webview';
import ProgressBar from "../../volt/ProgressBar.vue";

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'file-selected', file: File): void;
  (event: 'file-selected', file: File, currentDir: String): void;
}>();
const props = defineProps({
  progress: Number
})
const isFileSelected = ref(false);
const droppedFilePath = ref<string | null>(null);
let unlisten: (() => void) | null = null;

function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    isFileSelected.value = true;
    emit('file-selected', file);
  }
}

function onDrop(event: DragEvent) {
  event.preventDefault();
  const file = event.dataTransfer?.files?.[0];
  if (file) {
    isFileSelected.value = true;
    emit('file-selected', file);
  }
}

function onDragOver(event: DragEvent) {
  event.preventDefault(); // Important to allow dropping
}

onMounted(async () => {
  const webview = getCurrentWebview();

  unlisten = await webview.onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths;
      if (paths && paths.length > 0) {
        console.log('User dropped:', paths);
        isFileSelected.value = true;
        droppedFilePath.value = paths[0]; // store for preview
        // emit('file-selected', paths[0]);  // emit to parent
      }
    }
  });
});
onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>

<template>
  <div class="backdrop-blur-xl bg-upload fixed top-0 h-screen w-screen z-100">
    <div class="file-upload flex items-center justify-center w-full h-full">
      <i class="pi pi-times relative left-82 -top-40 sm:left-150 cursor-pointer" @click="$emit('close')"
         style="font-size: 30px"></i>
      <label for="dropzone-file"
             class="flex flex-col items-center content-center justify-center w-150 h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-neutral-800 dark:hover:bg-green-950 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
             @dragover="onDragOver"
             @drop="onDrop">
        <div class="flex flex-col items-center justify-center pt-5 pb-6">
          <svg class="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" aria-hidden="true"
               xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"/>
          </svg>
          <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">Click to upload</span> or
            drag and drop</p>
          <p class="text-xs text-gray-500 dark:text-gray-400">SVG, PNG, JPG or GIF (MAX. 800x400px)</p>
          <br>
          <p class="text-sm" v-if="isFileSelected && props.progress && props.progress < 1">Upload starting, please wait...</p>
          <p class="text-sm" v-if="props.progress &&  props.progress < 100 && props.progress > 1">Uploading:</p>
          <p class="text-sm" v-if="props.progress == 100">Upload completed!</p>
          <ProgressBar v-if="isFileSelected" class="mt-2 border-1 border-neutral-500 w-full" :show-value="true"
                       :value="Number(props.progress)"></ProgressBar>
        </div>
        <input id="dropzone-file" type="file" class="hidden" @change="onFileChange"/>
      </label>
    </div>
  </div>
</template>

<style scoped>
.bg-upload {
  background-color: hsla(0, 0%, 0%, 0.5);
}

.file-upload {
  position: fixed;
  z-index: 100;
}
</style>
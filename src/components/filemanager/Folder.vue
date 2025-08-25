<script setup lang="ts">
import { formatTextLimit } from '../../util/textFormats';

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  path: {
    type: String,
    required: true
  },
})
const emit = defineEmits(['delete'])
</script>

<template>
  <div class="folder-item rounded-2xl p-5 border-1 border-neutral-700 w-full h-full place-items-center relative">
    <i @click.stop="emit('delete')" class="right-2 top-2 p-1 delete-file pi pi-trash absolute z-10"
      style="font-size: 20px"></i>
    <i class="pi pi-folder" style="font-size: 40px" />
    <p class="hidden sm:block wrap-anywhere font-light" :title="props.name">
      {{ formatTextLimit(props.name, 30) }}
    </p>

    <p class="block sm:hidden wrap-anywhere font-light" :title="props.name">
      {{ formatTextLimit(props.name, 15) }}
    </p>
  </div>
</template>

<style scoped>
.folder-item {
  cursor: pointer;
  transition: background-color 0.5s ease;
}

.folder-item:hover {
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}

.delete-file {
  transition: color 0.5s ease;
}

.delete-file:hover {
  color: red;
}
</style>
<script setup lang="ts">
const props = defineProps({
  name: {
    type: String,
    required: true
  },
})
</script>

<template>
<div class="folder-item p-5 border-1 border-neutral-700 w-full h-fit place-items-center">
  <i class="pi pi-folder " style="font-size: 40px"/>
  <p>{{props.name}}</p>
</div>
</template>

<style scoped>
.folder-item{
  cursor: pointer;
  transition: background-color 0.5s ease;
}
.folder-item:hover {
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}
</style>
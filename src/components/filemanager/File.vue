<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref} from "vue";
import {useAppStore} from "@/stores/app.ts";
import { getFileTypeIcon } from "../../util/fileTypes";
const store = useAppStore();
const props = defineProps({
  name: {
    type: String,
    required: true
  },
  size:{
    type: Number,
    required: true
  },
})
const emit = defineEmits(['delete','link-generate'])
const showDropdown = ref(false);
const folderItemRef = ref<HTMLElement | null>(null);
function formatText(text: string){
  let limit = 30
  if(text.length > limit){
    return text.slice(0,limit-3)+"..."
  }else{
    return text;
  }
}
function calculateSize(size:number){ //size in bytes
  const kb = 1024;
  const mb = 1048576;
  const gb = 1073741824;
  const tb = 1099511627776;
  if(size < kb ){
    return (size).toFixed()+" B"
  }
  if(size < mb){
    return (size/kb).toFixed()+" KB"
  }
  if(size < gb){
    return (size/mb).toFixed(1)+" MB"
  }
  if(size < tb){
    return (size/gb).toFixed(2)+" GB"
  }
  if(size > tb){
    return (size/tb).toFixed(2)+" TB"
  }
}
function handleClickOutside(event: MouseEvent) {
  if (folderItemRef.value && !folderItemRef.value.contains(event.target as Node)) {
    showDropdown.value = false;
  }
}

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener("click", handleClickOutside);
});
</script>

<template>
  <div ref="folderItemRef" @click.right.prevent="showDropdown =  !showDropdown" class="folder-item relative rounded-2xl p-2 border-1 border-neutral-700 w-full h-full ">
      <div @click.stop="showDropdown =  !showDropdown;" class="pi pi-ellipsis-h p-1 menu rounded-2xl absolute z-10"></div>
      <div class="dropdown-menu absolute z-100 mt-5" :class="showDropdown ? 'translate-y-2 opacity-100 pointer-events-auto' : '-translate-y-5 opacity-0 pointer-events-none'">
        <ul class="bg-neutral-700 border-1 border-zinc-500">
          <li @click.stop="emit('delete')" class="p-2 dropdown-btn">Delete</li>
          <li @click.stop="emit('link-generate');showDropdown = false" class="p-2 dropdown-btn">Share link</li>
        </ul>
      </div>


    <i :class="getFileTypeIcon(props.name) || 'pi pi-file'" class="mt-3" style="font-size: 50px"/>
    <p class="wrap-anywhere font-light" :title="props.name" >{{formatText(props.name)}}</p>
    <p class="size font-extralight mt-2">{{calculateSize(props.size)}}</p>
    <p class="down-label hover:opacity-100 relative opacity-0 mt-0">download</p>
  </div>
</template>

<style scoped>
.size{
  font-size: 13px;
}
.folder-item{
  cursor: pointer;
  transition: background-color 0.5s ease;
}
.folder-item:hover{
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}
.folder-item:hover .down-label{
  opacity: 1;
}
.down-label{
  transition: opacity 0.5s ease;
}
.dropdown-menu{
  transition: all 300ms ease;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  right: 0;
}
.menu{
  transition: all 0.5s ease;
  right: 1rem;
}

.menu:hover {
  color:limegreen;
  background-color: rgba(255, 255, 255, 0.2);
}

.dropdown-btn{
  transition: color 0.5s ease;
}
.dropdown-btn:hover{
  background-color: rgba(255,255,255,0.5);
}
.delete-file{
  transition: color 0.5s ease;
}
.delete-file:hover{
  color: red;
}
</style>
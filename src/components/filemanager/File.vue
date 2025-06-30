<script setup lang="ts">

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  size:{
    type: Number,
    required: true
  }
})
const emit = defineEmits(['delete'])

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
</script>

<template>
  <div class="folder-item rounded-2xl p-2 border-1 border-neutral-700 w-full h-full place-items-center">
    <i @click.stop="emit('delete')" class="sm:ml-14 delete-file pi pi-trash absolute ml-12 z-10" style="font-size: 20px"></i>
    <i class="pi pi-file mt-3" style="font-size: 50px"/>
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
.delete-file{
  transition: color 0.5s ease;
}
.delete-file:hover{
  color: red;
}
</style>
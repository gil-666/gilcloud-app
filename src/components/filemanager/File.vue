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

function formatText(text: string){
  if(text.length > 20){
    return text.slice(0,17)+"..."
  }else{
    return text;
  }
}
function calculateSize(size:number){
  if(size > 1048576){
    return (size/1048576).toFixed()+" MB"
  }
}
</script>

<template>
  <div class="folder-item p-2 border-1 border-neutral-700 w-full h-full place-items-center">
    <i class="pi pi-file mt-3" style="font-size: 50px"/>
    <p class="wrap-anywhere font-light">{{formatText(props.name)}}</p>
    <p class="size font-extralight mt-2">{{calculateSize(props.size)}}</p>
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
.folder-item:hover {
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}
</style>
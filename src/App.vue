<script setup lang="ts">
import {ref} from "vue";
import Button from "./volt/Button.vue";
import Divider from "./volt/Divider.vue";
import ProgressBar from "./volt/ProgressBar.vue";
import {RouterView} from "vue-router";
import {Suspense} from "vue";
const folders = ref("");
const name = ref("");
const storageCount = ref({ //amount in MB
  "maxStorage": 15360,
  "currentUsage": 1000,
});
const progress = ref((storageCount.value.currentUsage/storageCount.value.maxStorage)*100);

</script>

<template>
  <main>
    <div class="header bg-neutral-900 w-full fixed z-50">
      <div class="header-content p-4 flex">
        <div class="logo w-42 place-items-center">
          <p class="text-2xl font-bold h-fit">GilCloud</p>
          <p class="bg-green-900 w-fit">Cloud Services</p>
        </div>
        <Divider layout="vertical"></Divider>

        <div class="logout-button absolute flex right-3 top-6">
          <Button class="upload-btn pi pi-upload mr-5"> Upload</Button>
          <Button label="Log Out"></Button>
        </div>
      </div>
    </div>
    <div class="flex relative h-screen">
      <div class="side-bar bg-neutral-900 min-w-40 max-w-50 text-center w-full flex-col z-0">
        <div class="side-bar-inside mt-30 w-full ">
          <Button class="btn-sidebar w-full mb-5" label="File Manager"></Button>
          <Button class="btn-sidebar w-full" label="VMs"></Button>
        </div>
        <div class="progress-bar mt-5 p-3 max-w-full bottom-10">
          <p class="storage-counter">{{(storageCount.currentUsage/1024).toFixed(1)}}GB of {{(storageCount.maxStorage/1024)}}GB used ({{progress.toFixed()}}%) </p>
          <ProgressBar class="border-1 border-neutral-500" :show-value="false" :value="progress"></ProgressBar>
        </div>
      </div>
      <div class="font-bold flex-col w-full mt-22">
        <Suspense>


        <RouterView />
        </Suspense>
      </div>
    </div>
  </main>
</template>

<style scoped>
.logo {
  text-align: center;
}

Button:hover {
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}

.upload-btn{
  background-color: #0d542b;
}

.btn-sidebar{
  transition: all 0.5s ease; /* Apply the transition to the base state */
  border: unset;
  border-radius: unset;
  background-color: #0d542b;
}
.storage-counter{
  font-size: 14px;
}
</style>
<style>


</style>
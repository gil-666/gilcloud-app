<script setup lang="ts" xmlns="http://www.w3.org/1999/html">
import {onMounted, ref} from "vue";
import Button from "./volt/Button.vue";
import Divider from "./volt/Divider.vue";
import ProgressBar from "./volt/ProgressBar.vue";
import Login from "./components/Login.vue";
import {RouterView} from "vue-router";
import {Suspense} from "vue";
import UploadDialog from "./components/filemanager/UploadDialog.vue";
import axios, {AxiosProgressEvent} from "axios";
import {useAppStore} from "./stores/app.ts";
const folders = ref("");

const name = ref("");
const storageCount = ref({ //amount in MB
  "maxStorage": localStorage.getItem("storage_total")/1048576,
  "currentUsage": localStorage.getItem("storage_used")/1048576,
});
const progress = ref((storageCount.value.currentUsage/storageCount.value.maxStorage)*100);
const uploadVisible = ref(false);
const selectedFile = ref<File | null>(null);
const uploadProgress = ref(0);
const isUploading = ref(false);
const isLoggedIn = ref(false);
const store = useAppStore();
onMounted(() => {
  // if (localStorage.getItem("username")){
  //   isLoggedIn.value = true;
  // }
  localStorage.clear() //forget logins for now
})

async function logOut(){
  isLoggedIn.value = false;
  localStorage.clear();
  window.location.reload()
}
async function uploadFile(file:File) {
  selectedFile.value = file;
  if (!selectedFile.value) return;

  const formData = new FormData();
  formData.append('file', selectedFile.value);
  // formData.append('dir',store.currentDir);
  isUploading.value = true;

  try {
    await axios.post('http://localhost:8080/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (e: AxiosProgressEvent) => {
        if (e.lengthComputable) {
          uploadProgress.value = Math.round((e.loaded * 100) / e.total);
        }
      },
    });
    console.log(selectedFile.value.name);
    console.log('Upload complete!');
  } catch (error) {
    console.error('Upload error:', error);
  } finally {
    isUploading.value = false;
  }
}
</script>

<template>
  <Login v-if="!isLoggedIn" @loginSuccess="isLoggedIn = true"></Login>
  <main>
    <div class="header bg-neutral-900 w-full fixed z-50">
      <div class="header-content p-4 flex">
        <div class="logo w-42 place-items-center">
          <p class="text-2xl font-bold h-fit">GilCloud</p>
          <p class="bg-green-900 w-fit">Cloud Services</p>
        </div>
        <Divider layout="vertical"></Divider>

        <div class="logout-button absolute flex right-3 top-6">
          <input class="sr-only" type="file" name="bruh">
          <Button @click="uploadVisible = !uploadVisible; console.log(uploadVisible)" class="upload-btn pi pi-upload mr-5"><p style="font-family: Inter,sans-serif">Upload</p></Button>
          <Button label="Log Out" @click="logOut"></Button>
        </div>
      </div>
    </div>
    <div class="flex relative h-screen">
      <div class="side-bar bg-neutral-900 min-w-40 max-w-50 text-center w-full flex-col z-0">
        <div class="side-bar-inside mt-30 w-full ">
          <Button class="btn-sidebar w-full mb-5" label="Drive"></Button>
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

  <UploadDialog v-if="uploadVisible" @close="uploadVisible = false" @file-selected="uploadFile"></UploadDialog>

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
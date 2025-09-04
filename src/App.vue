<script setup lang="ts" xmlns="http://www.w3.org/1999/html">
import { computed, onMounted, ref } from "vue";
import Button from "./volt/Button.vue";
import ProgressBar from "./volt/ProgressBar.vue";
import { RouterView, RouterLink, useRoute } from "vue-router";;
import UploadDialog from "./components/filemanager/UploadDialog.vue";
import axios, { AxiosProgressEvent } from "axios";
import { useAppStore } from "./stores/app.ts";
import Toast from './volt/Toast.vue';
import Loader from "@/components/Loader.vue";
import { useRouter } from "vue-router";
import VideoPlayer from "./components/media/VideoPlayer.vue";
import { useApiUrl } from "./main.ts";
const window2 = typeof window;
const isLoggedIn = ref(false);
const store = useAppStore();;
const storageCount = computed(() => store.storageCount);
const progress = computed(() => store.progress);
const uploadVisible = ref(false);
const selectedFile = ref<File | null>(null);
const uploadProgress = ref(0);
const isUploading = ref(false);
const toggleSidebar = () => (store.UIEvents.showMenuBar = !store.UIEvents.showMenuBar);
const router = useRouter();
const route = useRoute();

onMounted(() => {
  const isPublic = route.meta.public ?? false;

  if (!isPublic) {
    if (localStorage.getItem("username")) {
      isLoggedIn.value = true;
      updateStorageCount();
      setHomeDir();
    } else {
      router.push({
        path: '/login',
        query: { redirect: router.currentRoute.value.fullPath }
      });
    }
  } else {
    if (localStorage.getItem("username")) {
      isLoggedIn.value = true;
      updateStorageCount();
      setHomeDir();
    }
  }
});

async function updateStorageCount() {
  const response = await axios.get(`${useApiUrl()}/storage/${localStorage.getItem('username')}`);
  store.setStorageCount(response.data);
}

function setHomeDir() {
  const homeDir = localStorage.getItem('home_dir');
  if (homeDir) {
    store.setUserHomeDir(homeDir);
  }
}

async function logOut() {
  isLoggedIn.value = false;
  localStorage.clear();
  window.location.reload()
}

async function onSelectedFile(file: File) {
  await uploadFile(file);
  setHomeDir()
  updateStorageCount();
}

async function uploadFile(file: File) {
  selectedFile.value = file;
  if (!selectedFile.value) return;

  const formData = new FormData();
  formData.append('dir', store.currentDir.replace(/^"|"$/g, '')); //THE ORDER OF APPEND MATTERS
  formData.append('file', selectedFile.value);
  console.log("sent directory: ", store.currentDir)

  isUploading.value = true;

  try {
    await axios.post(`${useApiUrl()}/upload`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (e: AxiosProgressEvent) => {
        if (e.lengthComputable) {
          uploadProgress.value = Math.round((e.loaded * 100) / e.total!);
        }
      },
    });
    console.log(selectedFile.value.name);
    console.log('Upload complete!');

  } catch (error) {
    console.error('Upload error:', error);
  } finally {
    isUploading.value = false;
    setTimeout(() => {
      window.location.reload();
    }, 1000)

  }
}
</script>

<template>
  <!--  <Login v-if="!isLoggedIn" @loginSuccess="isLoggedIn = true;setHomeDir"></Login>-->
  <main>
    <div class="header bg-neutral-900 w-full fixed z-90">
      <div class="header-content place-items-center p-4 flex">
        <Button @click="toggleSidebar()" class="lg:hidden fixed z-30">
          ☰
        </Button>
        <div class="logo w-55 place-items-center flex">
          <div>
            <img class="ml-2 logo scale-110" width="56" src="@/assets/logtrans.png">
          </div>
          <div class="text-logo transition-all duration-250 opacity-100 not-sm:opacity-0 ">
            <p class="text-2xl font-bold h-fit">GilCloud</p>
            <!--            <p class="bg-green-900 w-fit">Cloud Services</p>-->
          </div>

        </div>
        <!--        <Divider layout="vertical"></Divider>-->

        <div class="logout-button absolute flex right-3 top-6">
          <input class="sr-only" type="file" name="bruh">
          <p class="content-center mr-4 sm:block hidden">Hello, {{ store.username }}!</p>
          <Button @click="uploadVisible = !uploadVisible; console.log(uploadVisible)"
            class="upload-btn pi pi-upload mr-5">
            <p style="font-family: Inter,sans-serif">Upload</p>
          </Button>
          <Button label="Log Out" @click="logOut"></Button>
        </div>
      </div>
    </div>
    <div class="flex relative h-screen">
      <div
        class="side-bar h-full fixed lg:relative z-20 bg-neutral-900 min-w-40 max-w-50 text-center w-full flex-col transition-transform duration-300"
        :class="store.UIEvents.showMenuBar ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'">
        <div class="side-bar-inside mt-30 w-full ">
          <div class="user-label -mt-5 mb-2">
            <p class="content-center not-sm:block hidden">Hello, {{ store.username }}!</p>
          </div>
          <router-link @click="toggleSidebar" to="/"><Button class="btn-sidebar w-full mb-5"
              label="Drive"></Button></router-link>
          <router-link @click="toggleSidebar" to="/movies"><Button class="btn-sidebar w-full"
              label="Movies"></Button></router-link>
        </div>
        <div class="progress-bar mt-5 p-3 max-w-full bottom-10">
          <p class="storage-counter">{{ (storageCount.currentUsage / 1024).toFixed(2) }}GB of
            {{ (storageCount.maxStorage / 1024) }}GB used ({{ progress }}%) </p>
          <ProgressBar class="border-1 border-neutral-500" :show-value="false" :value="Number(progress)"></ProgressBar>
        </div>
      </div>
      <div class="font-bold flex-col w-full mt-22">
        <Toast />
        <Suspense>
          <template #default>
            <RouterView />
          </template>
          <template #fallback>
            <Loader />
          </template>
        </Suspense>
      </div>
      
    </div>
  </main>

  <UploadDialog v-if="uploadVisible" @close="uploadVisible = false; (typeof window2 as any).location.reload()"
    @file-selected="onSelectedFile" :progress="uploadProgress"></UploadDialog>

</template>

<style scoped>
.text-logo {
  @media not (width >=41rem

    /* 640px */
  ) {
    /*CUSTOM PORQUE SI NO FUNCIONA OPACITY ANIM*/
    opacity: 0%;
  }
}

.logo-img {
  transform: scale(120%);
  text-align: center;
}

.logo {
  text-align: center;
}

Button:hover {
  background-color: grey;
  box-shadow: inset #f6f6f6 0 0 10px 2px;
}

.upload-btn {
  background-color: #0d542b;
}

.btn-sidebar {
  transition: all 0.5s ease;
  /* Apply the transition to the base state */
  border: unset;
  border-radius: unset;
  background-color: #0d542b;
}

.storage-counter {
  font-size: 14px;
}
</style>
<style></style>
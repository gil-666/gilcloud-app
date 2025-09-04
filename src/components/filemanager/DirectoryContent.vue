<template>
  <div>
    <AudioPlayer @close="audioActive = false" :visible="audioActive" :src="audioSource"></AudioPlayer>
    <ImageViewer @close="imageActive = false" v-if="imageActive" :src="imageSource"></ImageViewer>
    <VideoPlayer @close="videoActive = false" v-if="videoActive" :src="videoSource"></VideoPlayer>
    <FileView @close="fileViewActive = false" v-if="fileViewActive" :file="fileSource"></FileView>
    <p v-if="isDirEmpty()" class="font-light">No items to show</p>
    <!-- <p class="font-light" id="load">Loading</p> -->
    <div v-if="!isDirEmpty()"
      class="file-list gap-8 grid 2xl:grid-cols-8 lg:grid-cols-4 grid-cols-2 justify-items-center">
      <Folder v-for="folder in folders" :key="folder.path" v-bind="folder" @delete="deleteFile(folder.path)"
        @click="changeDir(folder.path)" />
      <File v-for="file in files" :key="file.path" v-bind="file" @click="performFileAction(file)"
        @delete="deleteFile(file.path)" @download="downloadFile(file.path)" @link-generate="onGenerateLink(file.path)"
        @link-generate-view="generateLinkView(file.path)" />
    </div>

  </div>

</template>

<script setup lang="ts">

import { ref, defineEmits, KeepAlive, Teleport } from "vue";
import axios from "axios";
import Folder from "../../components/filemanager/Folder.vue";
import File from "../../components/filemanager/File.vue";
import { useAppStore } from "../../stores/app.ts";
import { useToast } from "primevue";
import { FileStructure, getFileTypeString } from "../../util/fileTypes.ts";
import { generateViewLink, extractRelativePath, generateLink } from "../../util/linkGen.ts"
import AudioPlayer from "../media/AudioPlayer.vue";

const emit = defineEmits<{
  (e: "update:dir", dir: string): void;
}>();

const props = defineProps<{ dir: string }>();
const store = useAppStore();
const toast = useToast();
const folders = ref<any[]>([]);
const files = ref<any[]>([]);

let navHistory: string[] = [];

//video player
import VideoPlayer from "../../components/media/VideoPlayer.vue";
const videoActive = ref(false);
const videoSource = ref('');

//image viewer
import ImageViewer from "../../components/media/ImageViewer.vue";
import { useApiUrl } from "../../main.ts";
import FileView from "../media/FileView.vue";
const imageActive = ref(false);
const imageSource: FileStructure | any = ref(null)

//audio player

const audioActive = ref(false);
const audioSource = ref<FileStructure | null>(null)
const audioPlayerShow = ref(false);
//file preview view
const fileSource: FileStructure | any = ref('')
const fileViewActive = ref(false)

// helpers
function onGenerateLink(filePath: string) {
  const link = generateLink(filePath)
  if (link) {
    try {
      navigator.clipboard.writeText(link);
      toast.add({
        detail: `${link}`,
        summary: `Download link copied to clipboard`,
        severity: "success",
        life: 3000,
      });
    } catch (e) {
      console.error(e);
    }
  }

}
function generateLinkView(filePath: string) {
  const link = generateViewLink(filePath)
  try {
    navigator.clipboard.writeText(link!);
    toast.add({
      detail: `${link}`,
      summary: `Download link copied to clipboard`,
      severity: "success",
      life: 3000,
    });
  } catch (e) {
    console.error(e);
  }
}

async function downloadFile(filePath: string) {
  const username = localStorage.getItem("username");
  if (!username) return;
  const relativePath = extractRelativePath(filePath, username);
  const encodedPath = encodeURIComponent(relativePath);
  window.open(`${useApiUrl()}/download/${username}/${encodedPath}`);
}
async function deleteFile(filePath: string) {
  const username = localStorage.getItem("username");
  if (!username) return;
  const relativePath = extractRelativePath(filePath, username);
  const encodedPath = encodeURIComponent(relativePath);
  try {
    await axios.delete(`${useApiUrl()}/delete/${username}/${encodedPath}`);
    await loadDirectory(props.dir);
    toast.add({
      detail: `File ${encodedPath} was deleted successfully`,
      summary: "File deleted",
      severity: "info",
      life: 3000,
    });
    await updateStorageCount();
  } catch (e) {
    console.error("Delete failed:", e);
  }
}

async function changeDir(newDir: string) {
  navHistory.push(props.dir);
  const normalized = newDir.replace(/\\/g, "/");
  emit("update:dir", normalized);
}

// API helpers
async function getFolders(dir: string) {
  const response = await axios.get(`${useApiUrl()}/folders`, {
    params: { dir },
  });
  return response.data;
}

async function getFiles(dir: string) {
  const response = await axios.get(`${useApiUrl()}/files`, {
    params: { dir },
  });
  console.log(response.data);
  return response.data;
}

async function updateStorageCount() {
  const response = await axios.get(
    `${useApiUrl()}/storage/${localStorage.getItem("username")}`
  );
  store.setStorageCount(response.data);
}

const isDirEmpty = () => folders.value.length === 0 && files.value.length === 0

// suspend point: load on mount / whenever dir prop changes
const isLoading = ref(true);

async function loadDirectory(dir: string) {
  isLoading.value = true;
  folders.value = [];
  files.value = [];
  if (dir) {
    try {
      folders.value = await getFolders(dir);
      files.value = await getFiles(dir);
    } finally {
      isLoading.value = false;
    }
  }
}
await (async () => {
  await loadDirectory(props.dir); // Suspense will suspend here
})();

function loadVideo(filePath: string) {
  videoSource.value = filePath;
  videoActive.value = true;
}

function loadImage(file: FileStructure) {
  imageSource.value = file;
  imageActive.value = true;
}

function loadAudio(filePath: FileStructure) {
  audioSource.value = filePath;
  audioActive.value = true;
  audioPlayerShow.value = true;
}

function loadFileViewer(file: FileStructure) {
  fileSource.value = file;
  fileViewActive.value = true;
}

function performFileAction(file: FileStructure) {
  const fileType = getFileTypeString(file.name);
  if (fileType === "video") {
    loadVideo(file.path);
  } else if (fileType === "photo") {
    loadImage(file);
  } else if (fileType === "audio") {
    loadAudio(file);
  } else {
    loadFileViewer(file)
  }
}
// watch(
//     () => props.dir,
//     async (newDir, oldDir) => {
//       if (newDir && newDir !== oldDir) {
//         await loadDirectory();
//       }
//     }
// );
</script>

<style scoped>
/* scoped styles if any */
</style>

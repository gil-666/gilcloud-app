<template>
        <div>
          <p v-if="isDirEmpty()" class="font-light">No items to show</p>
          <!-- <p class="font-light" id="load">Loading</p> -->
          <div v-if="!isDirEmpty()" class="file-list gap-10 grid 2xl:grid-cols-8 lg:grid-cols-4 grid-cols-2 justify-items-center">
            <Folder
                v-for="folder in folders"
                :key="folder.path"
                v-bind="folder"
                @delete="deleteFile(folder.path)"
                @click="changeDir(folder.path)"
            />
            <File
                v-for="file in files"
                :key="file.path"
                v-bind="file"
                @click="downloadFile(file.path)"
                @delete="deleteFile(file.path)"
                @link-generate="generateLink(file.path)"
            />
          </div>
          
        </div>

</template>

<script setup lang="ts">
import { ref, defineEmits } from "vue";
import axios from "axios";
import Folder from "../../components/filemanager/Folder.vue";
import File from "../../components/filemanager/File.vue";
import { useAppStore } from "../../stores/app.ts";
import { useToast } from "primevue";

const emit = defineEmits<{
  (e: "update:dir", dir: string): void;
}>();

const props = defineProps<{ dir: string }>();
const store = useAppStore();
const toast = useToast();
const folders = ref<any[]>([]);
const files = ref<any[]>([]);

let navHistory: string[] = [];

// helpers
function extractRelativePath(fullPath: string, username: string): string {
  const marker = `/user/${username}/`;
  const normalized = fullPath.replaceAll("\\", "/"); // normalize slashes
  const index = normalized.indexOf(marker);
  if (index === -1) return ""; // username not found
  return normalized.slice(index + marker.length);
}

function generateLink(filePath: string) {
  const username = localStorage.getItem("username");
  if (!username) return;
  const relativePath = extractRelativePath(filePath, username);
  const encodedPath = encodeURIComponent(relativePath);
  const encodedLink = `${window.API_URL}/download/${username}/${encodedPath}`;
  try {
    navigator.clipboard.writeText(encodedLink);
    toast.add({
      detail: `${encodedLink}`,
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
  window.open(`${window.API_URL}/download/${username}/${encodedPath}`);
}

async function deleteFile(filePath: string) {
  const username = localStorage.getItem("username");
  if (!username) return;
  const relativePath = extractRelativePath(filePath, username);
  const encodedPath = encodeURIComponent(relativePath);
  try {
    await axios.delete(`${window.API_URL}/delete/${username}/${encodedPath}`);
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
  const normalized = newDir.replaceAll("\\", "/");
  emit("update:dir", normalized);
}

// API helpers
async function getFolders(dir: string) {
  const response = await axios.get(`${window.API_URL}/folders`, {
    params: { dir },
  });
  return response.data;
}

async function getFiles(dir: string) {
  const response = await axios.get(`${window.API_URL}/files`, {
    params: { dir },
  });
  console.log(response.data);
  return response.data;
}

async function updateStorageCount() {
  const response = await axios.get(
      `${window.API_URL}/storage/${localStorage.getItem("username")}`
  );
  store.setStorageCount(response.data);
}

const isDirEmpty = () => folders.value.length === 0 && files.value.length === 0

// suspend point: load on mount / whenever dir prop changes
async function loadDirectory(dir: string) {
  folders.value = [];
  files.value = [];
  folders.value = await getFolders(dir);
  files.value = await getFiles(dir);
}
await (async () => {
  await loadDirectory(props.dir); // Suspense will suspend here
})();


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

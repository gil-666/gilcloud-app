<template>
    <Loader v-if="!loaded" />
    <div v-else class="file-player-container fixed text-center inset-0 justify-center place-items-center z-100">
        <BannerLink v-if="!props.file" />
        <i v-if="!props.link" class="pi pi-times fixed right-4 top-4 cursor-pointer z-90" @click="$emit('close')"
            style="font-size: 30px"></i>
        <div :class="props.link ? 'w-full cont-view' : 'cont cont-w-normal'"
            class="p-5 relative h-screen items-center place-items-center">
            <div class="title mt-8">
                <h1 class="text-3xl max-w-full break-all">
                    {{ 'File' }}
                </h1>
            </div>

            <div class="p-5 inner-file rounded-2xl border-1 place-items-center items-center border-neutral-700 mt-5">
                <!-- <img :src="props.src?.path ? generateLink(props.src?.path) : props.link" alt="Image" class="image" /> -->
                <div class="pi pi-file relative" style="font-size: 40px;">
                    <div class="text-2xl font-bold mt-3 break-all">{{ formatTextLimit(file.name, 50) }}</div>
                </div>
            </div>
            <div @click="downloadFile(file.path)" class="download-btn cursor-pointer p-4 rounded-2xl shadow-2xl mt-5">
                Download</div>
            <div v-if="metadata" class="file-details mt-5">
                <div class="relative">
                    <p class="text-3xl">file details:</p>
                    <p class="text-lg">
                        Uploaded by:
                        {{ metadata.username }}
                    </p>
                    <p class="text-lg">
                        Type:
                        {{ metadata.fileType }}
                    </p>
                    <!-- <p class="text-lg">
                        Resolution: {{ metadata.width }}x{{ metadata.height }}
                    </p> -->
                    <br></br>
                </div>
            </div>

        </div>

    </div>
</template>
<script setup lang="ts">
import { useHead } from '@unhead/vue'
import { getFileTypeString, type FileStructure } from '../../util/fileTypes';
useHead({
    title: 'GilCloud | File',
    meta: [
        { name: 'description', content: 'ooooooo' },
        { property: 'og:title', content: 'GilCloud | File' },
        { property: 'og:description', content: 'Store your files in GilCloud.' },
        { property: 'og:type', content: 'website' },

    ]
})
import { ref, onMounted, computed, ComputedRef } from 'vue';
import Loader from '../Loader.vue';
import BannerLink from '../ui/BannerLink.vue';
import { useApiUrl } from '../../main';
import { extractRelativePath, generateLink, generateRelativePathString } from '../../util/linkGen';
import axios from 'axios';
import { formatTextLimit } from '../../util/textFormats';
let file: FileStructure;
const emit = defineEmits<{ (e: 'close'): void }>()
const loaded = ref(false);
const props = defineProps({
    file: { type: Object, required: false },
    link: { type: String, required: false }
})
let metadata = {
    username: 'Someone',
    fileType: '',
}
function downloadFile(link: string) {
    const gen = generateLink(link)
    window.open(gen)
}
onMounted(async () => {
    const filePath: string = props.file?.path || props.link;
    const marker = "../data/storage/user/";
    const decoded = decodeURIComponent(filePath);
    const index = decoded.indexOf(marker);

    let relative: string; // converts homepage paths and share link paths into URIs to fetch
    if (index !== -1) {
        relative = decoded.slice(index + marker.length);
    } else {
        relative = decoded;
    }

    // Extract username and path
    const [username, ...rest] = relative.split("/");
    const innerPath = rest.join("/");
    metadata.username = username;
    metadata.fileType = getFileTypeString(innerPath) ?? "File";
    try {
        let res = await axios.get(
            `${useApiUrl()}/file?username=${encodeURIComponent(username)}&path=${encodeURIComponent(innerPath)}`
        );
        // console.log(res);
        file = res.data;
        loaded.value = true;
    } catch (e) {
        console.error("Failed to fetch file info:", e);
    }
});
</script>
<style scoped>
.download-btn {
    background-color: rgb(0, 143, 0) !important;
}

.download-btn::hover {
    background-color: darkgreen;
}

.image {
    max-width: 100%;
    max-height: 650px;
    display: block;
    margin: 0 auto;
    border-radius: 10px;
}

.cont {
    max-width: 100%;
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.cont-w-normal{
    width: 700px;
}

.cont-view {
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.inner-file {
    width: 100%;
    height: 10rem;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    overflow: hidden;
}

.file-player-container {
    background-color: rgba(0, 0, 0, 0.7);
    transition: all 0.3s ease;
}

.file-player-container i {
    color: white;
    cursor: pointer;
    transition: color 0.3s ease;
}

.file-player-container i:hover {
    color: red;
}
</style>

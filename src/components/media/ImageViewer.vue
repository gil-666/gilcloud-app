<template>
    <Loader v-if="!loaded" />
    <div v-else class="photo-player-container fixed text-center inset-0 flex justify-center z-90">
        <i v-if="!props.link" class="pi pi-times fixed right-4 top-4 cursor-pointer z-90" @click="$emit('close')"
            style="font-size: 30px"></i>

        <div :class="props.link ? 'w-full cont-view' : 'cont'" class="p-5 relative h-screen">
            <div class="title mt-8">
                <h1 class="text-3xl max-w-full break-all">
                    {{ props.src?.name || 'Image' }}
                </h1>
            </div>

            <div class="p-5 inner-photo">
                <img :src="props.src?.path ? generateLink(props.src?.path) : props.link" alt="Image" class="image" />
            </div>

            <div v-if="metadata" class="photo-details mt-5">
                <div class="relative">
                    <p class="text-2xl">Photo details:</p>
                    <!-- <p class="text-sm">
                    Uploaded by:
                    {{ props.src.slice(props.src.indexOf("user/") + 5, props.src.lastIndexOf("/")) }}
                </p> -->
                    <p class="text-lg">
                        Resolution: {{ metadata.width }}x{{ metadata.height }}
                    </p>
                    <br></br>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import {useHead} from '@unhead/vue'
useHead({
  title: 'GilCloud | Image',
  meta: [
    { name: 'description', content: 'ooooooo' },
    { property: 'og:title', content: 'GilCloud | Image' },
    { property: 'og:description', content: 'View your photos in GilCloud.' },
    { property: 'og:type', content: 'website' },
    
  ]
})
import { ref, onMounted } from 'vue';
import Loader from '../Loader.vue';
import { generateLink } from '../../util/linkGen';
const emit = defineEmits<{ (e: 'close'): void }>()
const loaded = ref(false);
const props = defineProps({
    src: { type: Object, required: false },
    title: { type: String, required: false },
    link: { type: String, required: false },
})
const metadata = ref<any>(null)
onMounted(() => {
    const img = new Image();
    img.src = props.src?.path ? String(generateLink(props.src.path)) : String(props.link);
    img.onload = async () => {
        loaded.value = true;
        metadata.value = {
            width: img.width,
            height: img.height,
        };
    };
});
</script>
<style scoped>
.image {
    max-width: 100%;
    max-height: 650px;
    display: block;
    margin: 0 auto;
    border-radius: 10px;
}
.cont {
    max-width: 900px;
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.cont-view {
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.inner-photo {

    overflow: hidden;
}

.photo-player-container {
    background-color: rgba(0, 0, 0, 0.7);
    transition: all 0.3s ease;
}

.photo-player-container i {
    color: white;
    cursor: pointer;
    transition: color 0.3s ease;
}

.photo-player-container i:hover {
    color: red;
}
</style>

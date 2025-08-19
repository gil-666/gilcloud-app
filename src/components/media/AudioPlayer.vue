<template>
    <Loader v-if="!loaded" />
    <div v-show="loaded" class="audio-player-container fixed inset-0 flex justify-center z-90">
        <i class="pi pi-times fixed right-4 top-4 cursor-pointer z-100" @click="$emit('close')"
            style="font-size: 30px"></i>

        <div class="cont p-5 relative h-screen">
            <div class="title mt-8">
                <h1 class="text-3xl max-w-full break-all">
                    {{ props.src.name || 'Image' }}
                </h1>
            </div>

            <div class="p-5 inner-audio place-items-center">
                <audio :src="props.src.path || props.link" ref="audioPlayerRef" style="display: none" controlslist="nodownload"
                    controls class="audio" preload="auto" />
                <img ref="albumArtRef" src="../../assets/logtrans.png" alt="Audio" class="album-art" @mousemove="(e)=>(handleMouseMove(e,albumArtRef))" @mouseleave="() => resetTransform(albumArtRef)"  />
                <!-- progress -->
                <div class="progress-bar-cont flex items-center space-x-2">
                    <span class="text-sm w-12 text-right">{{ formatTime(currentTime) }}</span>

                    <input type="range" min="0" :max="duration" step="0.1" v-model="currentTime" @input="seekAudio"
                        class="flex-1 progress-bar" />
                </div>
                <i :class="isPlaying ? 'pi pi-pause' : 'pi pi-play'" class="play-button cursor-pointer mb-4"
                    @click="togglePlay"></i>
                <div v-if="metadata" class="audio-details mt-5">
                    <div class="relative">
                        <p class="text-2xl">audio details:</p>
                        <!-- <p class="text-sm">
                    Uploaded by:5
                    {{ props.src.slice(props.src.indexOf("user/") + 5, props.src.lastIndexOf("/")) }}
                </p> -->
                        <p class="text-lg">
                            audio duration: {{ (metadata.duration / 60).toFixed(2) }} minutes
                        </p>
                        <!-- <p class="text-lg">
                        audio bitrate: {{ metadata.bitrate }} kbps
                    </p> -->
                        <p class="text-lg">
                            audio format: {{ metadata.format }}
                        </p>
                        <br></br>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted, Ref } from 'vue';
import { handleMouseMove, resetTransform } from '../../util/anim';
import Loader from '../Loader.vue';
const loaded = ref(false);
const emit = defineEmits<{ (e: 'close'): void }>()
const props = defineProps({
    src: { type: Object, required: true },
    link: { type: String, required: false },
});
const metadata = ref<any>(null);
const audioPlayerRef = ref<HTMLAudioElement | null>(null);
const albumArtRef = ref<HTMLImageElement | null>(null);
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
let rafId: number | null = null;

const formatTime = (time: number) => {
  const minutes = Math.floor(time / 60)
    .toString()
    .padStart(2, "0");
  const seconds = Math.floor(time % 60)
    .toString()
    .padStart(2, "0");
  return `${minutes}:${seconds}`;
};
const togglePlay = () => {
  const audio = audioPlayerRef.value;
  if (!audio) return;

  if (audio.paused) {
    audio.play();
    isPlaying.value = true;
    updateProgress();
  } else {
    audio.pause();
    isPlaying.value = false;
    if (rafId) cancelAnimationFrame(rafId);
  }
};

// update progress bar
const updateProgress = () => {
  if (!audioPlayerRef.value) return;
  currentTime.value = audioPlayerRef.value.currentTime;
  rafId = requestAnimationFrame(updateProgress);
};

// seek audio
const seekAudio = () => {
  if (audioPlayerRef.value) {
    audioPlayerRef.value.currentTime = currentTime.value;
  }
};
onMounted(() => {
  
  const audio = audioPlayerRef.value;
  if (!audio) return;
  audio.onloadedmetadata = () => {
    togglePlay()
    loaded.value = true;
    duration.value = audio.duration;
    metadata.value = {
      duration: audio.duration,
      format: props.src.path.split(".").pop() || "unknown",
    };
    
  };

  audio.onended = () => {
    isPlaying.value = false;
    currentTime.value = 0;
  };
});

onUnmounted(() => {
  if (rafId) cancelAnimationFrame(rafId);
});
</script>

<style scoped>
.cont {
    width: 900px;
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.inner-audio {

    overflow: hidden;
}

.play-button {
    font-size: 30px;
    margin-top: 10px;
}

.audio-player-container {
    background-color: rgba(0, 0, 0, 0.7);
    transition: all 0.3s ease;
}

.audio-player-container i {
    color: white;
    cursor: pointer;
    transition: color 0.3s ease;
}

.audio-player-container i:hover {
    color: red;
}
.progress-bar-cont {
    width: 20rem;
}
.progress-bar {
    -webkit-appearance: none;
    width: 100%;
    height: 5px;
    background: #d1d1d1;
    border-radius: 5px;
    outline: none;
}
.progress-bar::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #0d542b;
    cursor: pointer;
}
.album-art {
    width: 250px;
    height: 250px;
    background-color: black;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    transition: all 0.3s ease;
    border-radius: 10px;
    margin-bottom: 20px;
}
</style>


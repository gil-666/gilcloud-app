<template>
  <Loader v-if="!loaded" />
  <div v-show="loaded" class="audio-player-container fixed text-center inset-0 flex justify-center z-100">
    <i v-if="!props.link" class="pi pi-times fixed right-4 top-4 cursor-pointer z-100" @click="$emit('close')"
      style="font-size: 30px"></i>

    <div :class="props.link ? 'w-full cont' : 'cont cont-w-normal'" class="p-5 relative h-screen place-items-center">
      <div class="title mt-3 mb-2">
        <h1 v-if="!metadata?.title" class="text-3xl max-w-full break-all">
          {{ fileName || props.link?.lastIndexOf('/') || 'Audio' }}
        </h1>
        <h1 v-else class="text-3xl max-w-full break-all">
          {{ metadata.title }}
        </h1>
        <h1 class="text-2xl max-w-full break-all mt-2">
          {{ metadata?.artist || 'Unknown Artist' }}
        </h1>
      </div>
      <img ref="albumArtRef" src="../../assets/logtrans.png" alt="Audio" class="album-art mt-8"
        @mousemove="(e) => (handleMouseMove(e, albumArtRef))" @mouseleave="() => resetTransform(albumArtRef)" />
      <div class="p-5 inner-audio place-items-center z-100">
        <audio ref="audioPlayerRef" style="display: none" controlslist="nodownload" controls class="audio"
          preload="auto" />
        <!-- progress -->
        <div class="progress-bar-cont relative items-center space-x-2">
          <input type="range" min="0" :max="duration" step="0.1" v-model="currentTime" @input="onSliderInput"
            @change="onSliderChange" class="flex-1 progress-bar" />
          <div class="timecode font-light absolute flex w-full space-x-1 mt-1">
            <div class="text-sm w-full text-left">{{ formatTime(currentTime) }}</div>
            <div class="text-sm"></div>
            <div class="text-sm w-full text-right">{{ formatTime(duration) }}</div>
          </div>
        </div>

        <i :class="isPlaying ? 'pi pi-pause' : 'pi pi-play'" class="p-1 play-button cursor-pointer mb-4"
          @click="togglePlay"></i>
        <div v-if="metadata" class="audio-details mt-5">
          <div class="relative">
            <p class="text-2xl">audio details:</p>
            <!-- <p class="text-sm">
                    Uploaded by:5
                    {{ props.src.slice(props.src.indexOf("user/") + 5, props.src.lastIndexOf("/")) }}
                </p> -->
            <p class="text-lg" v-if="metadata.album">
              Album: {{ metadata.album }}
            </p>
            <br>
            <p class="text-lg" v-if="metadata.genre">
              Genre: {{ metadata.genre }}
            </p>
            <p class="text-lg" v-if="metadata.year">
              Year: {{ metadata.year }}
            </p>

            <!-- <p class="text-lg">
                        audio bitrate: {{ metadata.bitrate }} kbps
                    </p> -->
            <p class="text-lg">
              Duration: {{ Math.floor(duration / 60)
                +
                ":" +
                (Math.floor(duration % 60).toString().padStart(2, "0")) }}
            </p>
            <p class="text-lg">
              Audio format: {{ audioFormat }}
            </p>
            <br></br>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import {useHead} from '@unhead/vue'
useHead({
  title: 'GilCloud | Audio Player',
  meta: [
    { name: 'description', content: 'play music o algo asi' },
    { property: 'og:title', content: 'GilCloud | Audio Player' },
    { property: 'og:description', content: 'Stream your favorite music directly from GilCloud.' },
    { property: 'og:type', content: 'website' }, // optional preview image
  ]
})
import { ref, onMounted, onUnmounted, Ref, watch, computed } from 'vue';
import { albumArtOnPause, albumArtOnPlay, applyGlow, handleMouseMove, resetTransform } from '../../util/anim';
import Loader from '../Loader.vue';
import { readTrackTags } from '../../helper/audioplayer';
import { generateLink } from '../../util/linkGen';
import { format } from 'path';
const loaded = ref(false);
const emit = defineEmits<{ (e: 'close'): void }>()
const props = defineProps({
  src: { type: Object, required: false },
  link: { type: String, required: false },
});
const metadata = ref<any>(null);
const audioPlayerRef = ref<HTMLAudioElement | null>(null);

const albumArtRef = ref<HTMLImageElement | null>(null);
const albumArtImg: Ref<string | null> = ref(null);
const isPlaying = ref(false);
const currentTime = ref(0);
const fileName = ref('')
const duration = ref(0);
const audioFormat = ref('')
const sliderTime = ref(0);
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
    audio.play()
      .then(() => {
        // Successfully started playback
        isPlaying.value = true;
        updateProgress();
        albumArtOnPlay(albumArtRef.value);
      })
      .catch((err) => {
        console.warn('Autoplay blocked until user interaction', err);
        // Ensure player is paused and UI reflects that
        audio.pause();
        isPlaying.value = false;
        albumArtOnPause(albumArtRef.value);
        if (rafId) cancelAnimationFrame(rafId);
      });
  } else {
    // Pause logic
    audio.pause();
    isPlaying.value = false;
    albumArtOnPause(albumArtRef.value);
    if (rafId) cancelAnimationFrame(rafId);
  }
};

const playAudio = async () => {
  const audio = audioPlayerRef.value;
  if (!audio) return;

  try {
    await audio.play();
    isPlaying.value = true;
    albumArtOnPlay(albumArtRef.value);
    updateProgress();
  } catch (err) {
    console.warn("Autoplay blocked, paused instead", err);
    audio.pause();
    isPlaying.value = false;
    albumArtOnPause(albumArtRef.value);
  }
};

const tryPlayWithRetry = async (retries = 2) => {
  const audio = audioPlayerRef.value;
  if (!audio) return;

  try {
    await audio.play();
    isPlaying.value = true;
    albumArtOnPlay(albumArtRef.value);
    updateProgress();
  } catch {
    if (retries > 0) {
      audio.load();
      setTimeout(() => tryPlayWithRetry(retries - 1), 200);
    }
  }
};

// update progress bar
const updateProgress = () => {
  if (!audioPlayerRef.value) return;
  currentTime.value = audioPlayerRef.value.currentTime;
  rafId = requestAnimationFrame(updateProgress);
};

// seek audio
function onSliderInput(e: Event) {
  sliderTime.value = Number((e.target as HTMLInputElement).value);
  currentTime.value = sliderTime.value;
}

function onSliderChange(e: Event) {
  if (audioPlayerRef.value) {
    audioPlayerRef.value.currentTime = sliderTime.value;
  }
}

async function loadMetadata() {
  const source = props.link || props.src?.path
  if (!source) return
  const meta = await readTrackTags(String(source))
  if (!meta) return
  metadata.value = meta
  if (albumArtRef.value && meta.picture) {
    albumArtRef.value.onload = async () => {
      await applyGlow(albumArtRef.value!);
    };
    albumArtRef.value.src = meta.picture;
  }

  console.log("met", metadata.value)
}
onMounted(() => {
  const audio = audioPlayerRef.value;
  if (!audio) return;
  let lastSrc: string | null = null;

  watch(
    () => props.src?.path ? generateLink(props.src?.path) : props.link,
    async (newSrc) => {
      if (!newSrc) return;

      const audio = audioPlayerRef.value;
      if (!audio) return;

      // Only reload if the src changed
      if (audio.src !== newSrc) {
        audio.pause();
        audio.src = newSrc;
        audio.load();

        try {
          // Try to play automatically
          await audio.play();
          isPlaying.value = true;
          albumArtOnPlay(albumArtRef.value);
          updateProgress();
        } catch (err) {
          console.warn("Autoplay blocked, user interaction needed", err);
          isPlaying.value = false;
          albumArtOnPause(albumArtRef.value);
        }
      }
    },
    { immediate: true }
  );
  audio.onloadedmetadata = async () => {
    // togglePlay()
    loaded.value = true;
    const name = audio.src.split('/').pop()?.split('?')[0];
    fileName.value = String(name)
    duration.value = audio.duration;
    const format = computed(() => {
      const src = props.src?.path ?? props.link; // use link if no src
      if (!src) return "unknown";          // handle undefined case
      const extMatch = src.match(/\.([^.?#]+)(?:[?#]|$)/);
      return extMatch ? extMatch[1].toLowerCase() : "unknown";
    });
    audioFormat.value = format.value

    await loadMetadata()
    playAudio()
  };

  audio.onended = () => {
    isPlaying.value = false;
    currentTime.value = 0;
    albumArtOnPause(albumArtRef.value);
  };

  audio.onerror = () => {
    console.error("Audio error:", audio.error);
  };
});

onUnmounted(() => {
  if (rafId) cancelAnimationFrame(rafId);
});


</script>

<style scoped>
.cont {
  background-color: rgb(41, 41, 41);
  overflow-y: auto;
  overflow-x: hidden;
}

.cont-w-normal {
  width: 800px;
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

.play-button:hover {
  color: rgb(145, 145, 145);
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
  -webkit-box-shadow: 0px 0px 80px 17px rgba(98, 255, 46, 0.9);
  -moz-box-shadow: 0px 0px 80px 17px rgba(98, 255, 46, 0.9);
  box-shadow: 0px 0px 80px 17px rgba(98, 255, 46, 0.9);
  transition: all 0.3s ease;
  border-radius: 10px;
  margin-bottom: 20px;
}
</style>

<template>
    <!-- <audio v-if="mediaData" :src="mediaData.src" ref="audioPlayerRef" @timeupdate="store.seek" @ended="store.isPlaying = false"></audio> -->
    <div
        class="background-player-wrapper banner fixed inset-0 place-content-center items-end flex pointer-events-none p-4 z-99">
        <div
            class="background-player-cont justify-between border-1 border-neutral-500 relative h-15 rounded-4xl pointer-events-auto">
            <div class="inside p-2 w-full h-full flex items-center gap-2">
                <div class="icon min-w-10 flex-col w-12 bg-black rounded-3xl">
                    <img class="rounded-3xl" :src="mediaData.picture ?? fallbackImage" alt="Logo" />
                </div>
                <div class="info w-45 flex-col text-sm">
                    <p class="font-light line-clamp-1">{{mediaData?.title}}</p>
                    <p class="font-bold line-clamp-1">{{mediaData?.artist}}</p>
                </div>

                <div class="progress-controls flex flex-col w-full items-center gap-2 -mb-2">
                    <div style="font-size: 20px" class="controls absolute top-0 space-x-2 justify-center text-lg mt-1">
                        <i class="pi pi-step-backward control-button cursor-pointer"></i>

                        <i @click="isPlaying = !isPlaying" :class="isPlaying ? 'pi pi-pause' : 'pi pi-play'" class="control-button play-button cursor-pointer rounded-3xl pt-1.5 pl-1.5 pb-1.5 pr-1.5 bg-white text-black"></i>
                        <i @click="store.stop" class="pi pi-stop control-button cursor-pointer"></i>
                        <i class="pi pi-step-forward control-button cursor-pointer"></i>
                    </div>
                    <div class="relative w-full">
                        <div class="absolute w-full">
                            <input v-model="store.playbackPosition" :max="mediaData.duration" type="range" class="progress-bar w-full" />
                            <div class="timecode absolute w-full -top-3 font-light text-sm">
                                <p class="absolute left-0">{{formatTime(store.playbackPosition)}}</p>
                                <p class="absolute right-0">{{ formatTime(mediaData.duration) }}</p>
                            </div>
                        </div>

                    </div>
                </div>
                <div class="w-4">

                </div>
            </div>

        </div>
    </div>

</template>
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { backgroundPlayerStore } from '../../stores/background-player';
import { formatTime } from '../../util/textFormats';
import { storeToRefs } from 'pinia';
import fallbackImage from '../../assets/logtrans.png';
import { on } from 'events';
const mediaData = computed(() => backgroundPlayerStore().currentTrack);
const store = backgroundPlayerStore();
const { isPlaying,playbackPosition } = storeToRefs(store);
const audioPlayerRef = ref<HTMLAudioElement | null>(null);
console.log(mediaData);
onMounted(() => {
    if (audioPlayerRef.value) {
        audioPlayerRef.value.currentTime = ref(playbackPosition).value;
    }
})
</script>
<style lang="css" scoped>
.background-player-cont {
  width: 100%;
  position: relative;
  backdrop-filter: blur(10px);
  overflow: hidden; 
}

.background-player-cont::before {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: linear-gradient(to bottom, #525252, #737373, #525252);
  opacity: 0.5;
  z-index: 0;
}
.background-player-cont > * {
  position: relative;
  z-index: 1;
}
.control-button {
    font-size: 18px;
    text-shadow: 0 0 1px black;
    transition: all 0.1s linear;
}
.control-button:hover {
    color: #14ad00;
    transform: scale(1.2);
}
.play-button{
    text-shadow: none;
    box-shadow: 0 0 1px #000000;
}
.play-button:hover {
    background: #31702e;
    color: white;
    transform: scale(1.2);
}
.progress-bar {
    -webkit-appearance: none;
    width: 100%;
    height: 6px;
    background: #d1d1d1;
    border-radius: 5px;
    outline: none;
}

.progress-bar::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #ffffff;
    cursor: pointer;
    transition: all 0.3s ease-in-out;
}


.progress-bar::-webkit-slider-thumb:hover {
    -webkit-appearance: none;
    appearance: none;
    width: 15px;
    height: 15px;
    background: #14ad00;
}
</style>
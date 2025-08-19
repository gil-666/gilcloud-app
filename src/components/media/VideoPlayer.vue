<template>
    <Loader v-if="!loaded" />
        <div ref="videoPlayerContainerRef" class="video-player-container fixed inset-0 flex justify-center z-100">
            <i v-if="!props.link" class="pi pi-times fixed right-4 top-4 cursor-pointer z-100" @click="$emit('close')"
                style="font-size: 30px"></i>
                <div :class="props.link ? 'w-full' : ''" class="cont relative h-screen place-items-center text-center">
                    <div class="w-full p-2 title mt-10 w-20">
                        <h1 v-if="!isHls && !props.link" class="text-3xl">
                            Now playing:
                            {{ videotitle }}
                        </h1>
                        <h1 v-else class="text-3xl">
                            Now playing:
                            {{ props.title || videotitle || 'Video' }}
                        </h1>
                    </div>

                    <div class="inner-video">
                        <video ref="videoPlayerRef" class="video-js vjs-default-skin" preload="auto"></video>
                    </div>

                    <div v-if="metadata" class="video-details mt-5">
                        <p class="text-lg">Video details:</p>
                        <p class="text-sm" v-if="!isHls && !props.link">
                            Uploaded by:
                            {{ props.src?.slice(props.src.indexOf("user/") + 5, props.src.lastIndexOf("/")) }}
                        </p>
                        <p class="text-sm">
                            Resolution: {{ metadata.width }}x{{ metadata.height }}
                        </p>
                        <p class="text-sm">
                            Duration: {{ (metadata.duration / 60).toFixed(2) }} minutes
                        </p>
                        <br></br>
                        <div v-if="audioTracks.length">
                            <p class="text-lg">Audio Tracks:</p>
                            <ul>
                                <li v-for="(track, idx) in audioTracks" :key="idx">
                                    {{ track.label || 'Track ' + (idx + 1) }} ({{ track.language || 'unknown' }})
                                </li>
                            </ul>
                        </div>
                        <br>
                        <div v-if="subtitleTracks.length">
                            <p class="text-lg">Subtitle Tracks:</p>
                            <ul>
                                <li v-for="(track, idx) in subtitleTracks" :key="idx">
                                    {{ track.label || 'Track ' + (idx + 1) }} ({{ track.language || 'unknown' }}) <span
                                        v-if="track.mode === 'showing'">(showing)</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, Ref, ref} from 'vue'
import { detectHls, detectMaster, updateStreamInfo, forceVhsQuality } from '../../helper/videoplayer.ts'
import videojs from 'video.js'
// videojs.log.level('debug');
import 'video.js/dist/video-js.css'
// import 'videojs-contrib-quality-levels'
import 'videojs-hls-quality-selector/dist/videojs-hls-quality-selector.js'
import 'videojs-hls-quality-selector/dist/videojs-hls-quality-selector.css'
import { useToast } from 'primevue'
import Loader from '../Loader.vue'
import Player from 'video.js/dist/types/player'
const loaded = ref(false)
const userLocale = navigator.language || 'en';
const languageDisplay = new Intl.DisplayNames([userLocale], { type: 'language' });
const userLangBase = userLocale.split('-')[0];
const media: any = ref()
const videotitle = ref('');
const emit = defineEmits<{ (e: 'close'): void }>()
const props = defineProps({
    src: { type: String, required: false },
    title: { type: String, required: false },
    subtracks: { type: Array as () => string[], default: () => [], required: false },
    link: { type: String, required: false },
})

const toast = useToast()

let player: any = null
const videoPlayerRef = ref<HTMLVideoElement | null>(null)
const metadata = ref({
    duration: 0,
    width: 0,
    height: 0
})
const videoPlayerContainerRef = ref<HTMLDivElement | null>(null)
const isHls: Ref<boolean | undefined> = ref(false)
const isMaster: Ref<boolean | undefined> = ref(false)
const audioTracks = ref<any[]>([])
const subtitleTracks = ref<any[]>([])
onMounted(() => {

    if (!videoPlayerRef.value) return
    isHls.value = detectHls(props.link ?? props.src)
    isMaster.value = detectMaster(props.link ?? props.src)


    player = videojs(videoPlayerRef.value, {
        autoplay: true,
        controls: true,
        html5: {
            vhs: {
                overrideNative: true // force VHS
            }
        },
        sources: [{
            src: props.src || props.link,
            type: isHls.value ? 'application/x-mpegURL' : 'video/mp4'
        }],
        controlBar: {
            pictureInPictureToggle: false,
        }

    })

    player.ready(() => {
        if (!isHls.value) return;
        // HLS-specific setup
        player.hlsQualitySelector({
            displayCurrentQuality: true,
        });
        const hlsPlugin = player.hlsQualitySelector();
        const origSetQuality = hlsPlugin.setQuality.bind(hlsPlugin);
        console.log(hlsPlugin)
        hlsPlugin.setQuality = function (quality: string) {
            console.log('Forcing VHS to quality:', quality);
            forceVhsQuality(player, quality);
            origSetQuality(quality);
        };

        let defaultTrackIndex = 0;
        //find users pref language
        props.subtracks.forEach((url: string, idx: number) => {
            const langCode = url.match(/subs_([a-z]{2})/)?.[1] || `lang${idx}`;
            const baseCode = langCode.split('-')[0]; // handle things like "en-US"

            if (baseCode.toLowerCase() === userLangBase.toLowerCase() && defaultTrackIndex === 0) {
                defaultTrackIndex = idx;
            }
        });

        props.subtracks.forEach((url: string, idx: number) => {
            const langCode = url.match(/subs_([a-z]{2})/)?.[1] || `lang${idx}`;
            let label;

            try {
                label = languageDisplay.of(langCode) || langCode;
            } catch {
                label = langCode;
            }

            player.addRemoteTextTrack({
                kind: 'subtitles',
                src: url,
                srclang: langCode,
                label,
                default: idx === defaultTrackIndex
            }, false);
        });

    })
    player.on('loadedmetadata', () => {
        updateStreamInfo(player, metadata)
        if(!videoPlayerContainerRef.value) return;
        loaded.value = true
        videoPlayerContainerRef.value.style.opacity = '1';
        // videotitle.value = player.mediainfo.name; 
        console.log("media", player?.getMedia())
        media.value = player?.getMedia()
        videotitle.value = decodeURIComponent(media.value.src[0].src.slice(media.value.src[0].src.lastIndexOf("/") + 1, media.value.src[0].src.length))
        // Audio tracks
        const aTracks = []
        const tracks = player.audioTracks && player.audioTracks()
        if (tracks && (tracks as any).length) {
            for (let i = 0; i < (tracks as any).length; i++) {
                const t = (tracks as any)[i]
                aTracks.push({
                    label: t.label,
                    language: t.language,
                    enabled: t.enabled
                })
            }
        }
        console.log('Audio tracks:', aTracks)
        audioTracks.value = aTracks

    })
    player.on('error', () => {
        console.log(props.src)
        console.error('Error loading video:', player.error())
        toast.add({
            severity: 'error',
            summary: 'Error',
            detail: 'Failed to load video.',
            life: 3000
        })
        emit('close')
    })
    player.on('timeupdate', () => updateStreamInfo(player, metadata));
    player.on('resize', () => updateStreamInfo(player, metadata));
})

onUnmounted(() => {
    if (player) {
        player.dispose()
    }
})
</script>

<style scoped>
@import 'video.js/dist/video-js.css';

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
    opacity: 1;
}

.cont {
    width: 1200px;
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;

}


.video-player-container {
    background-color: rgba(0, 0, 0, 0.7);
    transition: all 0.5s ease;
    opacity: 0;
}

/* .video-js::before {
    width: 1280px;
    height: 720px;
} */


.video-player-container i {
    color: white;
    cursor: pointer;
    transition: color 0.3s ease;
}

.video-player-container i:hover {
    color: red;
}
</style>
<style>
.inner-video {
  width: 100%;
  max-width: 100%;
  padding: 10px;
  aspect-ratio: 16 / 9;
  position: relative;
  overflow: hidden;
  border-radius: 10px;
  /* background-color: black; */
  
}

.inner-video .video-js {
    
  width: 100%;
  height: 100%;
  border-radius: inherit;
  overflow: hidden;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
}
.video-js video {
  object-fit: contain;
  width: 100%;
  height: 100%;

}
.video-js .vjs-control-bar{
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
}
.video-js .vjs-control-bar {
  background-color: rgba(0, 0, 0, 0.8); 
  border-top: 2px solid #43b839;      
  padding: 2px;
  height: 35px;
  right: -1px;
  width: auto;
  border-radius: 10px;
  transition: background-color 0.3s ease;
}

.video-js .vjs-progress-control:hover .vjs-time-tooltip {
    font-family: inherit;
    font-size: 10pt;
}
.video-js .vjs-control-bar .vjs-menu-content{
    font-family: inherit;
    background-color: rgba(0, 0, 0, 0.8);
    margin-bottom: 0.2rem;
    border-radius: 10px 10px 0 0;
}

.video-js .vjs-control-bar .vjs-menu-button-popup{
    transition: all 0.3 linear;
}

.video-js .vjs-control-bar .vjs-button {
  color: #ffffff;
}

.video-js .vjs-play-control:hover,
.video-js .vjs-volume-panel:hover,
.video-js .vjs-fullscreen-control:hover {
  color: #43b839;
}
</style>
<template>
    <div class="video-player-container fixed inset-0 flex justify-center z-100" @click="$emit('close')">
        <i class="pi pi-times fixed right-4 top-4 cursor-pointer z-100" @click="$emit('close')"
            style="font-size: 30px"></i>

        <div class="cont relative h-screen">
            <div class="title mt-10">
                <h1 v-if="!isHls" class="text-3xl">
                    Now playing:
                    {{ String(props.src).slice(props.src.lastIndexOf("/") + 1, props.src.length) }}
                </h1>
                <h1 v-else class="text-3xl">
                    Now playing:
                    {{ props.title || 'HLS Stream' }}
                </h1>
            </div>

            <div class="p-5 inner-video">
                <video ref="videoPlayerRef" class="video-js vjs-default-skin" preload="auto"></video>
            </div>

            <div v-if="metadata" class="video-details mt-5">
                <p class="text-lg">Video details:</p>
                <p class="text-sm" v-if="!isHls">
                    Uploaded by:
                    {{ props.src.slice(props.src.indexOf("user/") + 5, props.src.lastIndexOf("/")) }}
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
import { onMounted, onUnmounted, ref, VNodeRef, onBeforeUnmount } from 'vue'
import { detectHls, detectMaster, updateStreamInfo, forceVhsQuality } from '../../helper/videoplayer.ts'
import videojs from 'video.js'
// videojs.log.level('debug');
import 'video.js/dist/video-js.css'
// import 'videojs-contrib-quality-levels'
import 'videojs-hls-quality-selector/dist/videojs-hls-quality-selector.js'
import 'videojs-hls-quality-selector/dist/videojs-hls-quality-selector.css'
import { useToast } from 'primevue'
const emit = defineEmits<{ (e: 'close'): void }>()
const props = defineProps({
    src: { type: String, required: true },
    visible: { type: Boolean, default: false },
    title: { type: String, required: false },
    subtracks: { type: Array as () => string[], default: () => [], required: false },
})

const toast = useToast()

let player: any = null
const videoPlayerRef = ref<HTMLVideoElement | null>(null)
const videoPlayerContainerRef = ref<VNodeRef | null>(null)
const metadata = ref({
    duration: 0,
    width: 0,
    height: 0
})

const isHls = ref(false)
const isMaster = ref(false)
const audioTracks = ref<any[]>([])
const subtitleTracks = ref<any[]>([])
onMounted(() => {
    if (!videoPlayerRef.value) return

    isHls.value = detectHls(props.src)
    isMaster.value = detectMaster(props.src)


    player = videojs(videoPlayerRef.value, {
        autoplay: true,
        controls: true,
        html5: {
            vhs: {
                overrideNative: true // force VHS
            }
        },
        sources: [{
            src: props.src,
            type: isHls.value ? 'application/x-mpegURL' : 'video/mp4'
        }],

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
        // Agrega automáticamente cada .vtt desde props.subtracks
        props.subtracks.forEach((url: string, idx: number) => {
            const lang = url.match(/subs_([a-z]{2})/)?.[1] || `lang${idx}`
            const label = lang === 'sp' ? 'Español' : lang === 'en' ? 'English' : lang
            player.addRemoteTextTrack({
                kind: 'subtitles',
                src: url,
                srclang: lang,
                label: label,
                default: idx === 0
            }, false)
        });

    })
    player.on('loadedmetadata', () => {
        updateStreamInfo(player, metadata)

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

.cont {
    background-color: rgb(41, 41, 41);
    overflow-y: auto;
    overflow-x: hidden;
}

.inner-video {
    max-width: 1100px;
    max-height: 720px;
    overflow: hidden;
}

.video-player-container {
    background-color: rgba(0, 0, 0, 0.7);
    transition: all 0.3s ease;
}

.video-js {
    width: 100%;
    max-height: fit-content;
    border-radius: 10px;
    object-fit: contain;
    background-color: black;
    aspect-ratio: 16 / 9;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
}

.video-player-container i {
    color: white;
    cursor: pointer;
    transition: color 0.3s ease;
}

.video-player-container i:hover {
    color: red;
}
</style>

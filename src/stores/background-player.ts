import { defineStore } from "pinia";
import { ref } from "vue";

export const backgroundPlayerStore = defineStore('background-player', () => {
    const isPlaying = ref(false);
    const currentTrack = ref({
        title: 'Unknown Title',
        artist: 'Unknown Artist',
        album: 'Unknown Album',
        duration: 0, // in seconds
        picture: null, // URL to artwork image
        src: ''
    });
    const playbackPosition = ref(0); // in seconds
    const volume = ref(1); // range from 0.0 to 1.0
    const isMuted = ref(false);

    function play() {
        isPlaying.value = true;
        // Logic to start playback
    }

    function pause() {
        isPlaying.value = false;
        // Logic to pause playback
    }

    function stop() {
        isPlaying.value = false;
        playbackPosition.value = 0;
        // Logic to stop playback
    }

    function setTrack(track: { title: string; artist: string; album: string; duration: number; picture: any, src: string }) {
        currentTrack.value = track;
        playbackPosition.value = 0;
        // Logic to load the new track
    }

    function seek(position: number) {
        if (position >= 0 && position <= currentTrack.value.duration) {
            playbackPosition.value = position;
            // Logic to seek to the specified position
        }
    }

    function setVolume(vol: number) {
        if (vol >= 0 && vol <= 1) {
            volume.value = vol;
            // Logic to adjust volume
        }
    }

    function toggleMute() {
        isMuted.value = !isMuted.value;
        // Logic to mute/unmute audio
    }

    return {
        isPlaying,
        currentTrack,
        playbackPosition,
        volume,
        isMuted,
        play,
        pause,
        stop,
        setTrack,
        seek,
        setVolume,
        toggleMute,
    };
})
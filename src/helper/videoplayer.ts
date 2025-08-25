interface StreamMetadata {
    value: {
        duration: number;
        width: number;
        height: number;
    };
}
interface QualityLevel {
    width: number;
    height: number;
    enabled: boolean;
}

function detectHls(src: string | undefined) {
    // Simple detection for .m3u8 extension
    
    return src?.toLowerCase().endsWith('.m3u8')
}

function detectMaster(src: string | undefined) {
    // If filename contains 'master.m3u8'
    return src?.toLowerCase().includes('master.m3u8')
}

function updateStreamInfo(player: any, metadata: StreamMetadata): void {
    metadata.value = {
        duration: player.duration() ?? 0,
        width: player.videoWidth(),
        height: player.videoHeight(),
    }
}

function forceVhsQuality(player: any, selectedQuality: string) {
    const levels = Array.from(player.qualityLevels().levels_ as QualityLevel[]).filter((l: QualityLevel) => l.width && l.height);
    console.log('Forcing VHS to quality:', selectedQuality);
    console.log('Available quality levels:', levels);

    if (selectedQuality === 'auto') {
        levels.forEach((l: QualityLevel) => l.enabled = true);
        return; // VHS auto-adaptive
    }

    // Disable all levels except the selected one
    levels.forEach((l: QualityLevel) => {
        const pixels = Math.min(l.width, l.height);
        l.enabled = pixels === Number(selectedQuality);
    });

    // Access VHS playlists
    const vhs = player.tech_?.vhs;
    if (!vhs) return;

    // Find target playlist by resolution
    const targetPlaylist = vhs.playlists.main.playlists.find((p: any) => {
        const res = p.attributes.RESOLUTION;
        if (!res) return false;
        const pixels = Math.min(res.width, res.height);
        return pixels === Number(selectedQuality);
    });

    if (targetPlaylist) {
        // Set VHS media directly
        vhs.playlists.media(targetPlaylist);
        console.log('VHS playlist switched to', selectedQuality);
    } else {
        console.warn('Selected quality not found in VHS playlists');
    }
}

function setupVideoMediaSession(metadata: any, video: HTMLVideoElement) {
    if (!('mediaSession' in navigator)) return

    navigator.mediaSession.metadata = new MediaMetadata({
        title: metadata.name || 'Unknown Title',
        artist: 'GilCloud | Movies',
        album: 'GilCloud | Movies',
        artwork: [
            {
                src: metadata.cover || '/assets/logtrans.png',
                sizes: '512x512',
                type: 'image/jpg'
            }
        ]
    })

    // Actions for media controls
    navigator.mediaSession.setActionHandler('play', () => video.play())
    navigator.mediaSession.setActionHandler('pause', () => video.pause())
    navigator.mediaSession.setActionHandler('seekbackward', (details) => {
        video.currentTime = Math.max(video.currentTime - (details.seekOffset || 10), 0)
    })
    navigator.mediaSession.setActionHandler('seekforward', (details) => {
        video.currentTime = Math.min(video.currentTime + (details.seekOffset || 10), video.duration)
    })
    navigator.mediaSession.setActionHandler('previoustrack', null) // optional
    navigator.mediaSession.setActionHandler('nexttrack', null)     // optional
}
export {
    detectHls,
    detectMaster,
    updateStreamInfo,
    forceVhsQuality,
    setupVideoMediaSession
}
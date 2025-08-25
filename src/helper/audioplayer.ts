import { useApiUrl } from "../main";
import { watch } from 'vue'

async function readTrackTags(fileUrl: string | undefined): Promise<Record<string, any> | null> {
  if (!fileUrl) return null;

  try {
    const apiHost = useApiUrl();
    let path: string;
    if (fileUrl.startsWith(apiHost)) {
      path = new URL(fileUrl).pathname;
    } else {
      path = fileUrl;
    }

    const res = await fetch(
      `${apiHost}/music_metadata?file=${encodeURIComponent(path)}`
    );
    console.log(res)

    if (!res.ok) {
      console.error("Failed to fetch metadata:", res.status, res.statusText);
      return null;
    }

    const tags = await res.json();
    return tags;
  } catch (err) {
    console.error("Error fetching metadata:", err);
    return null;
  }
}


function setupMediaSession(metadata: any, audio: HTMLAudioElement) {
  if (!('mediaSession' in navigator)) return

  navigator.mediaSession.metadata = new MediaMetadata({
    title: metadata.title || 'Unknown Title',
    artist: metadata.artist || 'Unknown Artist',
    album: 'GilCloud | Audio Player',
    artwork: [
      {
        src: metadata.picture || '/assets/logtrans.png',
        sizes: '512x512',
        type: 'image/png'
      }
    ]
  })

  // Actions for media controls
  navigator.mediaSession.setActionHandler('play', () => audio.play())
  navigator.mediaSession.setActionHandler('pause', () => audio.pause())
  navigator.mediaSession.setActionHandler('seekbackward', (details) => {
    audio.currentTime = Math.max(audio.currentTime - (details.seekOffset || 10), 0)
  })
  navigator.mediaSession.setActionHandler('seekforward', (details) => {
    audio.currentTime = Math.min(audio.currentTime + (details.seekOffset || 10), audio.duration)
  })
  navigator.mediaSession.setActionHandler('previoustrack', null) // optional
  navigator.mediaSession.setActionHandler('nexttrack', null)     // optional
}

export { readTrackTags, setupMediaSession };

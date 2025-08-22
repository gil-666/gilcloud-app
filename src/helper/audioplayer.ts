import { useApiUrl } from "../main";

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

export { readTrackTags };

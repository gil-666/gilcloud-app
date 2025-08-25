import { useApiUrl } from "../main";
import { getFileTypeString } from "./fileTypes"

function extractRelativePath(fullPath: string, username: string): string {
  const marker = `/user/${username}/`;
  const normalized = fullPath.replace(/\\/g, "/"); // normal 
  const index = normalized.indexOf(marker);
  if (index === -1) return ""; // username not found
  return normalized.slice(index + marker.length);
}

function generateRelativePathString(filePath: string) {
  const parts = filePath.replace(/\\/g, "/").split("/");
  const userIndex = parts.indexOf("user");
  if (userIndex === -1 || userIndex + 1 >= parts.length) {
    console.error("Invalid file path format:", filePath);
    return;
  }

  const username = parts[userIndex + 1];
  const relativePath = parts.slice(userIndex + 2).join("/");

  const encodedPath = encodeURIComponent(relativePath);
  return `${username}/${encodedPath}`;
}


function generateViewLink(filepath:string){
    const type = getFileTypeString(filepath)
    switch (type) {
        case "photo":
            return `${window.origin}/view/photo?link=${generateRelativePathString(filepath)}`
        case "video":
            return `${window.origin}/view/video?link=${generateRelativePathString(filepath)}`
        case "audio":
            return `${window.origin}/view/audio?link=${generateRelativePathString(filepath)}`
        default:
            return `${window.origin}/view/file?link=${generateRelativePathString(filepath)}`
    }
}
function generateLink(filePath: string | any) {
  const username = localStorage.getItem("username");
  if (!username) return;
  const relativePath = extractRelativePath(filePath, username);
  const encodedPath = encodeURIComponent(relativePath);
  const encodedLink = `${useApiUrl()}/download/${username}/${encodedPath}`;
  return encodedLink
}
export{
    generateViewLink,
    generateRelativePathString,
    extractRelativePath,
    generateLink
}
import fileTypes from "../assets/file-types.json"
interface FileStructure {
  name: string;
  path: string;
  size: number;
}
interface MovieStructure {
  id: number;
  title: string;
  master: string;
  cover: string;
  audiotracks: string[];
  subtracks: string[];
}
function getFileTypeIcon(filename: string) {
  const extension = filename.slice(filename.lastIndexOf("."),filename.length)
//   console.log("file", extension)
  for (const type of Object.values(fileTypes)) {
        if (type.extensions.includes(extension)) {
            return type.icon;
        }
    }
    return null;
}
function getFileTypeString(filename: string) {
  const extension = filename.slice(filename.lastIndexOf("."),filename.length)
//   console.log("file", extension)
  for (const [name, type] of Object.entries(fileTypes)) {
        if (type.extensions.includes(extension)) {
            return name
        }

  }
    return null;
}
export {getFileTypeIcon,getFileTypeString}
export type {FileStructure, MovieStructure}
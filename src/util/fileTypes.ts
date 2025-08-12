import fileTypes from "../assets/file-types.json"

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
export {getFileTypeIcon}
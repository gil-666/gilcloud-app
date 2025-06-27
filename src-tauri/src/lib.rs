// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::io;
use std::path::Path;
use serde_json::json;

#[derive(serde::Serialize)]
struct FileEntry {
    name: String,
    path: String,
    size: u64,
}
#[derive(serde::Serialize)]
struct FolderEntry{
    name: String,
    path: String,
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_folders(dir: String) -> Result<Vec<FolderEntry>, String> {
    let path = Path::new(&dir);
    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            Some(FolderEntry {
                name,
                path: path.to_string_lossy().to_string(),
            })
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
fn get_files(dir: String) -> Result<Vec<FileEntry>, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(&dir);
    let entries = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file()) // Only files
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let metadata = entry.metadata().ok()?;
            let size = metadata.len();
            Some(FileEntry {
                name,
                path: path.to_string_lossy().to_string(),
                size,
            })
        })
        .collect();

    Ok(entries)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_folders,get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

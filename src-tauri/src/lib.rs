// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs as fsb;
use std::io;
use std::path::Path;
use sanitize_filename::sanitize;
use serde_json::json;
use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpServer, middleware, HttpResponse, Error};
use actix_cors::Cors;
use futures_util::TryStreamExt as _;
use std::io::Write;
use tokio::fs;
use tokio::io::AsyncWriteExt;

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
#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Ensure the storage directory exists
    let storage_dir = "./storage/user";
    if !Path::new(storage_dir).exists() {
        fs::create_dir_all(storage_dir).await?;
    }

    // Iterate multipart stream
    while let Some(item) = payload.try_next().await? {
        let mut field = item;

        // Only process fields named "file" (adjust if needed)
        let content_disposition = field.content_disposition();

        let filename = if let Some(filename) = content_disposition.get_filename() {
            sanitize(&filename)
        } else {
            // Fallback filename
            format!("file-{}", uuid::Uuid::new_v4())
        };

        let filepath = format!("{}/{}", storage_dir, filename);

        // Create or overwrite file
        let mut f = fs::File::create(filepath).await?;

        // Stream file chunks to disk
        while let Some(chunk) = field.try_next().await? {
            f.write_all(&chunk).await?;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_folders(dir: String) -> Result<Vec<FolderEntry>, String> {
    let path = Path::new(&dir);
    let entries = fsb::read_dir(path)
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
#[tokio::main]
pub async fn run() {
    // Spawn actix server in background
    println!("running server...");
    let server = tokio::spawn(async {
        HttpServer::new(|| {
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600),
                )
                .wrap(middleware::Logger::default())
                .service(upload)
        })
            .bind(("0.0.0.0", 8080))
            .expect("Failed to bind Actix server")
            .run()
            .await
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_folders,get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


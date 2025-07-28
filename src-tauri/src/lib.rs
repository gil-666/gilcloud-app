mod db;
use db::{register,login,storage_usage};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs as fsb;
use std::io;
use std::path::Path;
use sanitize_filename::sanitize;
use actix_files::NamedFile;
use serde_json::json;
use actix_multipart::Multipart;
use actix_web::{get, post, delete, web, App, HttpServer, middleware, HttpResponse, Error, error, Responder};
use std::path::PathBuf;
use actix_cors::Cors;
use futures_util::TryStreamExt as _;
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
#[derive(serde::Deserialize)]
struct DownloadQuery {
    path: String,
}
#[derive(serde::Deserialize)]
struct CreateFolderRequest {
    parent_dir: String,
    folder_name: String,
}
#[get("/download/{username}/{filename:.*}")]
async fn download_file(
    path: web::Path<(String, String)>,
) -> Result<NamedFile, actix_web::Error> {
    let (username, filename) = path.into_inner();
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
        .join("storage")
        .join("user");
    let base_dir = base_dir.canonicalize()?;
    let full_path = base_dir.join(&username).join(&filename).canonicalize()?;

    if !full_path.starts_with(&base_dir) {
        return Err(actix_web::error::ErrorForbidden("Access denied"));
    }

    Ok(NamedFile::open(full_path)?.set_content_disposition(
        actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![],
        },
    ))
}
#[delete("/delete/{username}/{filename}")]
async fn delete_file(
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, actix_web::Error> {
    let (username, filename) = path.into_inner();

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
        .join("storage")
        .join("user");
    let base_dir = base_dir.canonicalize()?;

    let full_path = base_dir.join(&username).join(&filename).canonicalize()?;

    // Security: Prevent directory traversal
    if !full_path.starts_with(&base_dir) {
        return Err(actix_web::error::ErrorForbidden("Access denied"));
    }

    // Delete file
    match fs::metadata(&full_path).await {
        Ok(metadata) => {
            println!("{}",&full_path.display());
            if metadata.is_file() {
                fs::remove_file(&full_path).await.map_err(|e| {
                    eprintln!("Error deleting file: {}", e);
                    error::ErrorInternalServerError("Failed to delete file")
                })?;
                Ok(HttpResponse::Ok().body("File deleted"))
            } else if metadata.is_dir() {
                fs::remove_dir_all(&full_path).await.map_err(|e| {
                    eprintln!("Error deleting directory: {}", e);
                    error::ErrorInternalServerError("Failed to delete directory")
                })?;
                Ok(HttpResponse::Ok().body("Directory deleted"))
            } else {
                Err(error::ErrorBadRequest("Unsupported file type"))
            }
        }
        Err(_) => Err(error::ErrorNotFound("File or directory not found")),
    }
}
#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Default storage directory (in case "dir" field is not found)
    let default_dir = "../data/storage/user".to_string();
    let dir = Arc::new(Mutex::new(default_dir.clone()));

    // Iterate multipart stream
    while let Some(item) = payload.try_next().await? {
        let mut field = item;
        println!("Field received: {}", field.name()); // 💡 shows each part

        // Get the name of the field (e.g., "file", "dir")
        let name = field.name().to_string();


        if name == "dir" {
            // Collect the "dir" value from the multipart field
            let mut value = Vec::new();
            while let Some(chunk) = field.try_next().await? {
                value.extend_from_slice(&chunk);
            }
            let dir_str = String::from_utf8(value).unwrap_or(default_dir.clone());
            *dir.lock().await = dir_str;
        }

        if name == "file" {
            // Ensure the storage directory exists
            let storage_dir = dir.lock().await.clone();
            if !Path::new(&storage_dir).exists() {
                fs::create_dir_all(&storage_dir).await?;
            }

            let content_disposition = field.content_disposition();

            let filename = if let Some(filename) = content_disposition.get_filename() {
                sanitize(&filename)
            } else {
                format!("file-{}", uuid::Uuid::new_v4())
            };

            let filepath = format!("{}/{}", storage_dir, filename);

            let mut f = fs::File::create(filepath).await?;

            while let Some(chunk) = field.try_next().await? {
                f.write_all(&chunk).await?;
            }
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[get("/folders")]
async fn folders(web::Query(params): web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let dir = match params.get("dir") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().body("Missing 'dir' parameter"),
    };

    let path = Path::new(dir);
    let entries: Vec<FolderEntry> = match fsb::read_dir(path) {
        Ok(read_dir) => read_dir
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_string_lossy().to_string();
                Some(FolderEntry {
                    name,
                    path: path.to_string_lossy().to_string(),
                })
            })
            .collect(),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read directory: {}", e)),
    };

    HttpResponse::Ok().json(entries)
}

#[get("/files")]
async fn files(web::Query(params): web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let dir = match params.get("dir") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().body("Missing 'dir' parameter"),
    };

    let path = Path::new(dir);
    let entries: Vec<FileEntry> = match fsb::read_dir(path) {
        Ok(read_dir) => read_dir
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_string_lossy().to_string();
                let size = entry.metadata().ok()?.len();
                Some(FileEntry {
                    name,
                    path: path.to_string_lossy().to_string(),
                    size,
                })
            })
            .collect(),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read directory: {}", e)),
    };

    HttpResponse::Ok().json(entries)
}
#[post("/create_folder")]
async fn create_folder(
    data: web::Json<CreateFolderRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
        .join("storage")
        .join("user");
    let base_dir = base_dir.canonicalize()?; // Resolve symlinks, normalize

    let target_dir = PathBuf::from(&data.parent_dir).join(&data.folder_name);
    let target_dir = target_dir.canonicalize().unwrap_or(target_dir);

    // Prevent path traversal
//     if !target_dir.starts_with(&base_dir) {
//         return Err(actix_web::error::ErrorForbidden("Access denied"));
//     }

    // Create the folder
    match fs::create_dir_all(&target_dir).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Folder created")),
        Err(e) => {
            eprintln!("Error creating folder: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Failed to create folder"))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // Spawn actix server in background
    println!("running server...");
    let db_pool = db::init_db().await;

    let server = tokio::spawn({
        let db_pool = db_pool.clone(); // clone the pool for the move
        async move {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db_pool.clone())) // now valid
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .wrap(middleware::Logger::default())
                    .service(upload)
                    .service(register)
                    .service(login)
                    .service(download_file)
                    .service(storage_usage)
                    .service(folders)
                    .service(files)
                    .service(delete_file)
                    .service(create_folder)
            })
                .bind(("0.0.0.0", 8080))
                .expect("Failed to bind Actix server")
                .run()
                .await
                .expect("Server run failed");
        }
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


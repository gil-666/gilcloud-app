mod db;
use actix_web::HttpRequest;
use db::{login, register, storage_usage};
use dunce;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::{
    delete, error, get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use futures_util::TryStreamExt as _;
use lofty::file::TaggedFileExt;
use sanitize_filename::sanitize;
use serde_json::json;
use sqlx::SqlitePool;
use std::fs as fsb;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(serde::Serialize)]
struct FileEntry {
    name: String,
    path: String,
    size: u64,
}
#[derive(serde::Serialize)]
struct FolderEntry {
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
#[derive(Debug, sqlx::FromRow)]
struct Movie {
    id: i64,
    title: String,
    path: String,
}

#[derive(serde::Serialize)]
struct MovieResponse {
    id: i64,
    title: String,
    master: String,
    cover: Option<String>,
    audiotracks: Vec<String>,
    subtracks: Vec<String>,
}

#[derive(serde::Serialize)]
struct MusicMetadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<u32>,
    genre: Option<String>,
    comment: Option<String>,
    track: Option<u32>,
    picture: Option<String>, // base64 data URI
}

#[get("/movies")]
pub async fn movies(
    db: web::Data<SqlitePool>,
    web::Query(params): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let movies_dir = Path::new("../data/storage/movies");

    // Helper to scan tracks and map to /movies/{id}/ URLs
    async fn scan_tracks(folder_path: &Path, id: i64) -> (Vec<String>, Vec<String>) {
        let mut audiotracks = Vec::new();
        let mut subtracks = Vec::new();

        if let Ok(mut entries) = tokio::fs::read_dir(folder_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let fname = entry.file_name().to_string_lossy().to_lowercase();
                if fname.starts_with("audio_") && fname.ends_with(".m3u8") {
                    if let Some(fname_os) = entry.path().file_name() {
                        let url = format!("/movies/{}/{}", id, fname_os.to_string_lossy());
                        audiotracks.push(url);
                    }
                }
                if fname.starts_with("subs_") && fname.ends_with(".vtt") {
                    if let Some(fname_os) = entry.path().file_name() {
                        let url = format!("/movies/{}/{}", id, fname_os.to_string_lossy());
                        subtracks.push(url);
                    }
                }
            }
        }

        (audiotracks, subtracks)
    }

    if let Some(id_str) = params.get("id") {
        // Fetch single movie by id
        let id = match id_str.parse::<i64>() {
            Ok(i) => i,
            Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
        };

        let movie = sqlx::query_as::<_, Movie>("SELECT id, title, path FROM movies WHERE id = ?")
            .bind(id)
            .fetch_optional(db.get_ref())
            .await;

        let movie = match movie {
            Ok(Some(m)) => m,
            Ok(None) => return HttpResponse::NotFound().body("Movie not found"),
            Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
        };

        let folder_path = Path::new(&movie.path);
        let master_url = format!("/movies/{}/master.m3u8", movie.id);

        let cover_path = folder_path.join("cover.jpg");
        let cover = if cover_path.exists() {
            Some(format!("/movies/{}/cover.jpg", movie.id))
        } else {
            None
        };

        let (audiotracks, subtracks) = scan_tracks(folder_path, movie.id).await;

        let response = MovieResponse {
            id: movie.id,
            title: movie.title,
            master: master_url,
            cover,
            audiotracks,
            subtracks,
        };

        return HttpResponse::Ok().json(response);
    }

    // No ID: scan all movies directory
    let mut responses = Vec::new();

    if let Ok(mut entries) = tokio::fs::read_dir(movies_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().is_dir() {
                let folder_path = entry.path();
                let title = folder_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                // Check if movie exists in DB
                let existing =
                    sqlx::query_as::<_, Movie>("SELECT id, title, path FROM movies WHERE path = ?")
                        .bind(folder_path.to_string_lossy())
                        .fetch_optional(db.get_ref())
                        .await;

                let movie = match existing {
                    Ok(Some(m)) => m,
                    Ok(None) => {
                        // Insert movie if missing
                        let res = sqlx::query("INSERT INTO movies (title, path) VALUES (?, ?)")
                            .bind(&title)
                            .bind(folder_path.to_string_lossy())
                            .execute(db.get_ref())
                            .await;

                        match res {
                            Ok(r) => Movie {
                                id: r.last_insert_rowid(),
                                title: title.clone(),
                                path: folder_path.to_string_lossy().to_string(),
                            },
                            Err(e) => {
                                eprintln!("DB insert error: {}", e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("DB query error: {}", e);
                        continue;
                    }
                };

                let master_url = format!("/movies/{}/master.m3u8", movie.id);
                let cover_path = folder_path.join("cover.jpg");
                let cover = if cover_path.exists() {
                    Some(format!("/movies/{}/cover.jpg", movie.id))
                } else {
                    None
                };

                let (audiotracks, subtracks) = scan_tracks(&folder_path, movie.id).await;

                responses.push(MovieResponse {
                    id: movie.id,
                    title: movie.title,
                    master: master_url,
                    cover,
                    audiotracks,
                    subtracks,
                });
            }
        }
    }

    HttpResponse::Ok().json(responses)
}

#[get("/movies/{id}/{filename:.*}")]
async fn movie_file(
    req: HttpRequest,
    db: web::Data<SqlitePool>,
    path: web::Path<(i64, String)>,
) -> actix_web::Result<NamedFile> {
    let (id, filename) = path.into_inner();

    // Lookup movie folder path from DB
    let movie = sqlx::query!("SELECT path FROM movies WHERE id = ?", id)
        .fetch_optional(db.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Movie not found"))?;

    let folder = PathBuf::from(
        movie
            .ok_or_else(|| actix_web::error::ErrorNotFound("Movie not found"))?
            .path
            .ok_or_else(|| actix_web::error::ErrorNotFound("Movie path missing"))?,
    );
    let full_path = folder.join(filename);

    // Prevent path traversal
    let canonical = dunce::canonicalize(&full_path)
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid path"))?;

    Ok(NamedFile::open(canonical)?)
}

#[get("/music_metadata")]
async fn music_metadata(
    web::Query(params): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    use lofty::file::AudioFile;
    use lofty::tag::Accessor;
    use std::borrow::Cow;
    use urlencoding::decode;
    // Decode the file param and ensure it's a Cow<'_, str>
    let file_param: Cow<'_, str> = match params.get("file") {
        Some(f) => decode(f).unwrap_or_else(|_| f.into()), // decode returns Cow<'_, str>
        None => return HttpResponse::BadRequest().body("Missing 'file' parameter"),
    };

    // Determine actual file path
    let file_path: PathBuf = if file_param.starts_with("/download/") {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("data")
            .join("storage")
            .join("user")
            .canonicalize()
            .unwrap();

        let rel_path = &file_param["/download/".len()..];
        let full_path = base_dir.join(rel_path);

        match full_path.canonicalize() {
            Ok(canon) if canon.starts_with(&base_dir) => canon,
            _ => return HttpResponse::Forbidden().body("Access denied"),
        }
    } else {
        PathBuf::from(file_param.as_ref())
    };

    let tagged_file = match lofty::read_from_path(&file_path) {
        Ok(f) => f,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Metadata error: {}", e)),
    };

    let tag = match tagged_file.primary_tag() {
        Some(t) => t,
        None => {
            return HttpResponse::Ok().json(MusicMetadata {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                comment: None,
                track: None,
                picture: None,
            })
        }
    };

    let mut meta = MusicMetadata {
        title: tag.title().map(|s| s.to_string()),
        artist: tag.artist().map(|s| s.to_string()),
        album: tag.album().map(|s| s.to_string()),
        year: tag.year(),
        genre: tag.genre().map(|s| s.to_string()),
        comment: tag.comment().map(|s| s.to_string()),
        track: tag.track(),
        picture: None,
    };

    if let Some(picture) = tag.pictures().first() {
        let mime_str = picture
            .mime_type()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let encoded = base64::encode(picture.data());
        meta.picture = Some(format!("data:{};base64,{}", mime_str, encoded));
    }

    HttpResponse::Ok().json(meta)
}

#[get("/download/{username}/{filename:.*}")]
async fn download_file(
    req: HttpRequest,
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

    let mut file = NamedFile::open(full_path)?;

    // Use inline if client requests Range (likely a player)
    let disposition_type = if req.headers().contains_key("Range") {
        actix_web::http::header::DispositionType::Inline
    } else {
        actix_web::http::header::DispositionType::Attachment
    };

    file = file.set_content_disposition(actix_web::http::header::ContentDisposition {
        disposition: disposition_type,
        parameters: vec![],
    });

    Ok(file)
}
#[delete("/delete/{username}/{filename}")]
async fn delete_file(path: web::Path<(String, String)>) -> Result<HttpResponse, actix_web::Error> {
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
            println!("{}", &full_path.display());
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
async fn folders(
    web::Query(params): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
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
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to read directory: {}", e))
        }
    };

    HttpResponse::Ok().json(entries)
}

#[get("/files")]
async fn files(
    web::Query(params): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
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
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to read directory: {}", e))
        }
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
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create folder",
            ))
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
                    .service(movies)
                    .service(movie_file)
                    .service(download_file)
                    .service(storage_usage)
                    .service(folders)
                    .service(files)
                    .service(delete_file)
                    .service(create_folder)
                    .service(music_metadata)
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

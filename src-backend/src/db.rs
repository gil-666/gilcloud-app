use std::{env, fs, io};
use std::path::{Path, PathBuf};
// src/db.rs
use actix_web::{get,post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Sqlite, Row};
use sqlx::migrate::MigrateDatabase;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use tokio::task;

#[derive(Serialize)]
struct UserInfo {
    username: String,
    storage_total: i64,
    storage_used: i64,
    home_dir: String,
}
#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}
pub async fn get_user_storage_info(db: &SqlitePool, username: &str) -> Result<(i64, i64), io::Error> {
    // Define path
    let home_dir_path = format!("../data/storage/user/{}", username);

    // Calculate used size
    let used_result = task::spawn_blocking(move || calculate_directory_size(&home_dir_path))
        .await
        .unwrap_or(Err(io::Error::new(io::ErrorKind::Other, "Size calc failed")))?;

    // Fetch total from DB
    let row = sqlx::query("SELECT storage_total FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(db)
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch total"))?;

    let total: i64 = row.try_get("storage_total").unwrap_or(0);

    Ok((total, used_result as i64))
}
#[get("/storage/{username}")]
pub async fn storage_usage(
    db: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> impl Responder {
    let username = path.into_inner();

    match get_user_storage_info(db.get_ref(), &username).await {
        Ok((total, used)) => {
            // Update DB (optional, comment out if not needed here)
            if let Err(e) = sqlx::query("UPDATE users SET storage_used = ? WHERE username = ?")
                .bind(used)
                .bind(&username)
                .execute(db.get_ref())
                .await
            {
                eprintln!("DB update failed: {}", e);
            }

            HttpResponse::Ok().json(serde_json::json!({
                "maxStorage": total/1048576,
                "currentUsage": used/1048576
            }))
        }
        Err(e) => {
            eprintln!("Error getting storage info: {}", e);
            HttpResponse::InternalServerError().body("Could not retrieve storage data")
        }
    }
}
pub async fn register_user(pool: &SqlitePool, username: &str, password: &str) -> Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .to_string();

    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn verify_user(pool: &SqlitePool, username: &str, password: &str) -> Result<bool> {
    let user = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    if let Some(record) = user {
        let password_hash: String = record.try_get("password_hash")?;
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    } else {
        Ok(false)
    }
}
// --- Actix route handlers ---

#[post("/register")]
pub async fn register(
    db: web::Data<SqlitePool>,
    data: web::Json<AuthData>,
) -> impl Responder {
    match register_user(&db, &data.username, &data.password).await {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(e) => {
            println!("Register error: {}", e);
            HttpResponse::BadRequest().body("Registration failed (maybe duplicate user)")
        }
    }
}

#[post("/login")]
pub async fn login(
    db: web::Data<SqlitePool>,
    data: web::Json<AuthData>,
) -> impl Responder {
    match verify_user(&db, &data.username, &data.password).await {
        Ok(true) => {
            // Define home directory path
            let home_dir_path = format!("../data/storage/user/{}", data.username);
            let home_dir_clone = home_dir_path.clone();
            // Ensure the directory exists
            if let Err(e) = fs::create_dir_all(&home_dir_path) {
                eprintln!("Could not create home dir: {}", e);
                return HttpResponse::InternalServerError().body("Failed to prepare user directory");
            }

            // Calculate storage used asynchronously
            let username = data.username.clone();
            let size_result = task::spawn_blocking(move || calculate_directory_size(&home_dir_path))
                .await
                .unwrap_or(Err(io::Error::new(io::ErrorKind::Other, "Size calc failed")));

            let storage_used = match size_result {
                Ok(size) => size as i64,
                Err(e) => {
                    eprintln!("Error calculating storage size: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to read user storage");
                }
            };

            // Update DB with new storage_used
            if let Err(e) = sqlx::query("UPDATE users SET storage_used = ? WHERE username = ?")
                .bind(storage_used)
                .bind(&data.username)
                .execute(db.get_ref())
                .await
            {
                eprintln!("Error updating storage_used: {}", e);
                return HttpResponse::InternalServerError().body("Failed to update storage usage");
            }

            // Fetch user data
            let user_row = sqlx::query(
                "SELECT username, storage_total FROM users WHERE username = ?"
            )
                .bind(&data.username)
                .fetch_one(db.get_ref())
                .await;

            match user_row {
                Ok(row) => {
                    let user_info = UserInfo {
                        username: row.try_get("username").unwrap_or_default(),
                        storage_total: row.try_get("storage_total").unwrap_or(0),
                        storage_used,
                        home_dir: home_dir_clone,
                    };
                    HttpResponse::Ok().json(user_info)
                }
                Err(e) => {
                    eprintln!("Error fetching user after login: {}", e);
                    HttpResponse::InternalServerError().body("Could not load user info")
                }
            }
        }
        Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => {
            eprintln!("Login failed: {}", e);
            HttpResponse::InternalServerError().body("Login error")
        }
    }
}

pub async fn init_db() -> SqlitePool {
    // Get current dir (likely 'src-tauri')
    let current_dir = env::current_dir().expect("Failed to get current dir");

    // Get parent (project root)
    let project_root = current_dir.parent().expect("No parent directory found");

    // Build absolute path to database file outside src-tauri
    let mut db_path = PathBuf::from(project_root);
    db_path.push("data/users.db");

    // Create the 'data' folder if missing
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create data directory");
        }
    }

    let storage_dirs = ["movies", "music", "games"];
    for folder in &storage_dirs {
        let mut path = PathBuf::from(project_root);
        path.push(format!("data/storage/{}", folder));
        if !path.exists() {
            fs::create_dir_all(&path).expect(&format!("Failed to create {:?}", path));
        }
    }

    // Construct database url with absolute path
    let database_url = format!("sqlite://{}", db_path.display());
    // Create the database file if it doesn't exist
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("Creating DB at {}", database_url);
        Sqlite::create_database(&database_url).await.unwrap();
    }

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite DB");

    // Create table if not exists (can also use migrations instead)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            type TEXT NOT NULL DEFAULT 'user',
            storage_total INTEGER NOT NULL DEFAULT 32212254720,
            storage_used INTEGER NOT NULL DEFAULT 0
        )
        "#
    )
        .execute(&pool)
        .await
        .expect("Failed to create users table");

    pool
}

pub fn calculate_directory_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    fn dir_size(dir: &Path) -> io::Result<u64> {
        let mut size = 0;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                size += metadata.len();
            } else if metadata.is_dir() {
                size += dir_size(&entry.path())?;
            }
        }
        Ok(size)
    }

    dir_size(path.as_ref())
}
